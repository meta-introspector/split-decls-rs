// Generated macro for macro_399 (macro)
macro_rules! Depcrate_x86_64macro_399 {
() => {
// Module: crate::x86_64
// Provides: {"macro_399"}
// Dependencies: {}
intrinsics ! { # [unsafe (naked)] # [cfg (any (all (windows , target_env = "gnu") , target_os = "cygwin" , target_os = "uefi"))] pub unsafe extern "custom" fn ___chkstk_ms () { core :: arch :: naked_asm ! ("push   %rcx" , "push   %rax" , "cmp    $0x1000,%rax" , "lea    24(%rsp),%rcx" , "jb     1f" , "2:" , "sub    $0x1000,%rcx" , "test   %rcx,(%rcx)" , "sub    $0x1000,%rax" , "cmp    $0x1000,%rax" , "ja     2b" , "1:" , "sub    %rax,%rcx" , "test   %rcx,(%rcx)" , "pop    %rax" , "pop    %rcx" , "ret" , options (att_syntax)) ; } }
};
}
