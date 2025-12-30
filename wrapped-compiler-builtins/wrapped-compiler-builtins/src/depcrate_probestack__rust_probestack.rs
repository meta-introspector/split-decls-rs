// Generated macro for __rust_probestack (function)
macro_rules! Depcrate_probestack__rust_probestack {
() => {
// Module: crate::probestack
// Provides: {"__rust_probestack"}
// Dependencies: {}
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
};
}
