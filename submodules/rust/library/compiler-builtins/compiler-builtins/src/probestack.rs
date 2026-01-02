# ! [doc = " This module defines the `__rust_probestack` intrinsic which is used in the"] # ! [doc = " implementation of \"stack probes\" on certain platforms."] # ! [doc = ""] # ! [doc = " The purpose of a stack probe is to provide a static guarantee that if a"] # ! [doc = " thread has a guard page then a stack overflow is guaranteed to hit that"] # ! [doc = " guard page. If a function did not have a stack probe then there's a risk of"] # ! [doc = " having a stack frame *larger* than the guard page, so a function call could"] # ! [doc = " skip over the guard page entirely and then later hit maybe the heap or"] # ! [doc = " another thread, possibly leading to security vulnerabilities such as [The"] # ! [doc = " Stack Clash], for example."] # ! [doc = ""] # ! [doc = " [The Stack Clash]: https://blog.qualys.com/securitylabs/2017/06/19/the-stack-clash"] # ! [doc = ""] # ! [doc = " The `__rust_probestack` is called in the prologue of functions whose stack"] # ! [doc = " size is larger than the guard page, for example larger than 4096 bytes on"] # ! [doc = " x86. This function is then responsible for \"touching\" all pages relevant to"] # ! [doc = " the stack to ensure that that if any of them are the guard page we'll hit"] # ! [doc = " them guaranteed."] # ! [doc = ""] # ! [doc = " The precise ABI for how this function operates is defined by LLVM. There's"] # ! [doc = " no real documentation as to what this is, so you'd basically need to read"] # ! [doc = " the LLVM source code for reference. Often though the test cases can be"] # ! [doc = " illuminating as to the ABI that's generated, or just looking at the output"] # ! [doc = " of `llc`."] # ! [doc = ""] # ! [doc = " Note that `#[naked]` is typically used here for the stack probe because the"] # ! [doc = " ABI corresponds to no actual ABI."] # ! [doc = ""] # ! [doc = " Finally it's worth noting that at the time of this writing LLVM only has"] # ! [doc = " support for stack probes on x86 and x86_64. There's no support for stack"] # ! [doc = " probes on any other architecture like ARM or PowerPC64. LLVM I'm sure would"] # ! [doc = " be more than welcome to accept such a change!"] # ! [cfg (not (feature = "mangled-names"))] # ! [cfg (not (any (windows , target_os = "cygwin")))] # ! [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] use split_decls_genesis :: ourprelude :: * ; #[cfg (target_arch = "x86_64")] #[unsafe (naked)] #[rustc_std_internal_symbol] pub unsafe extern "custom" fn __rust_probestack () { core :: arch :: naked_asm ! ("
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
    " , #[cfg (not (all (target_env = "sgx" , target_vendor = "fortanix")))] "       ret" , #[cfg (all (target_env = "sgx" , target_vendor = "fortanix"))] "
            // for this target, [manually patch for LVI].
            //
            // [manually patch for LVI]: https://software.intel.com/security-software-guidance/insights/deep-dive-load-value-injection#specialinstructions
            pop %r11
            lfence
            jmp *%r11
    " , "
            .cfi_endproc
    " , options (att_syntax)) } #[cfg (all (target_arch = "x86" , not (target_os = "uefi")))] #[unsafe (naked)] #[rustc_std_internal_symbol] pub unsafe extern "custom" fn __rust_probestack () { core :: arch :: naked_asm ! ("
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
    " , options (att_syntax)) } #[cfg (all (target_arch = "x86" , target_os = "uefi"))] #[unsafe (naked)] #[rustc_std_internal_symbol] pub unsafe extern "custom" fn __rust_probestack () { core :: arch :: naked_asm ! ("
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