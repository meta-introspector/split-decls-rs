
macro_rules! __rust_probestack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_probestack in module {}", module_path!());
    };
}

mkfn!{
    __rust_probestack_introspect!();
    # [cfg (target_arch = "x86_64")] # [unsafe (naked)] # [rustc_std_internal_symbol] pub unsafe extern "custom" fn __rust_probestack () { core :: arch :: naked_asm ! ("
            .cfi_startproc
            pushq  %rbp
            .cfi_adjust_cfa_offset 8
            .cfi_offset %rbp, -16
            movq   %rsp, %rbp
            .cfi_def_cfa_register %rbp

            mov    %rax,%r11        // duplicate %rax as we're clobbering %r11

            // Main loop, taken in one page increments. We're decrementing rsp by
            // a page each time until there's less than a page remaining. We're
            // guaranteed that this function isn't called unless there's more than a
            // page needed.
            //
            // Note that we're also testing against `8(%rsp)` to account for the 8
            // bytes pushed on the stack originally with our return address. Using
            // `8(%rsp)` simulates us testing the stack pointer in the caller's
            // context.

            // It's usually called when %rax >= 0x1000, but that's not always true.
            // Dynamic stack allocation, which is needed to implement unsized
            // rvalues, triggers stackprobe even if %rax < 0x1000.
            // Thus we have to check %r11 first to avoid segfault.
            cmp    $0x1000,%r11
            jna    3f
        2:
            sub    $0x1000,%rsp
            test   %rsp,8(%rsp)
            sub    $0x1000,%r11
            cmp    $0x1000,%r11
            ja     2b

        3:
            // Finish up the last remaining stack space requested, getting the last
            // bits out of r11
            sub    %r11,%rsp
            test   %rsp,8(%rsp)

            // Restore the stack pointer to what it previously was when entering
            // this function. The caller will readjust the stack pointer after we
            // return.
            add    %rax,%rsp

            leave
            .cfi_def_cfa_register %rsp
            .cfi_adjust_cfa_offset -8
    " , # [cfg (not (all (target_env = "sgx" , target_vendor = "fortanix")))] "       ret" , # [cfg (all (target_env = "sgx" , target_vendor = "fortanix"))] "
            // for this target, [manually patch for LVI].
            //
            // [manually patch for LVI]: https://software.intel.com/security-software-guidance/insights/deep-dive-load-value-injection#specialinstructions
            pop %r11
            lfence
            jmp *%r11
    " , "
            .cfi_endproc
    " , options (att_syntax)) }
}

macro_rules! __rust_probestack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_probestack in module {}", module_path!());
    };
}

mkfn!{
    __rust_probestack_introspect!();
    # [cfg (all (target_arch = "x86" , not (target_os = "uefi")))] # [unsafe (naked)] # [rustc_std_internal_symbol] pub unsafe extern "custom" fn __rust_probestack () { core :: arch :: naked_asm ! ("
            .cfi_startproc
            push   %ebp
            .cfi_adjust_cfa_offset 4
            .cfi_offset %ebp, -8
            mov    %esp, %ebp
            .cfi_def_cfa_register %ebp
            push   %ecx
            mov    %eax,%ecx

            cmp    $0x1000,%ecx
            jna    3f
        2:
            sub    $0x1000,%esp
            test   %esp,8(%esp)
            sub    $0x1000,%ecx
            cmp    $0x1000,%ecx
            ja     2b

        3:
            sub    %ecx,%esp
            test   %esp,8(%esp)

            add    %eax,%esp
            pop    %ecx
            leave
            .cfi_def_cfa_register %esp
            .cfi_adjust_cfa_offset -4
            ret
            .cfi_endproc
    " , options (att_syntax)) }
}

macro_rules! __rust_probestack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_probestack in module {}", module_path!());
    };
}

mkfn!{
    __rust_probestack_introspect!();
    # [cfg (all (target_arch = "x86" , target_os = "uefi"))] # [unsafe (naked)] # [rustc_std_internal_symbol] pub unsafe extern "custom" fn __rust_probestack () { core :: arch :: naked_asm ! ("
            .cfi_startproc
            push   %ebp
            .cfi_adjust_cfa_offset 4
            .cfi_offset %ebp, -8
            mov    %esp, %ebp
            .cfi_def_cfa_register %ebp
            push   %ecx
            push   %edx
            mov    %eax,%ecx

            cmp    $0x1000,%ecx
            jna    3f
        2:
            sub    $0x1000,%esp
            test   %esp,8(%esp)
            sub    $0x1000,%ecx
            cmp    $0x1000,%ecx
            ja     2b

        3:
            sub    %ecx,%esp
            test   %esp,8(%esp)
            mov    4(%ebp),%edx
            mov    %edx, 12(%esp)
            add    %eax,%esp
            pop    %edx
            pop    %ecx
            leave

            sub   %eax, %esp
            .cfi_def_cfa_register %esp
            .cfi_adjust_cfa_offset -4
            ret
            .cfi_endproc
    " , options (att_syntax)) }
}