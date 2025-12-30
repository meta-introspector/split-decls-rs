// Generated macro for define_syscall (macro)
macro_rules! Depcratedefine_syscall {
() => {
// Module: crate
// Provides: {"define_syscall"}
// Dependencies: {}
# [cfg (not (any (target_feature = "static-syscalls" , all (target_arch = "bpf" , feature = "unstable-static-syscalls"))))] # [macro_export] macro_rules ! define_syscall { (fn $ name : ident ($ ($ arg : ident : $ typ : ty) ,*) -> $ ret : ty) => { extern "C" { pub fn $ name ($ ($ arg : $ typ) ,*) -> $ ret ; } } ; (fn $ name : ident ($ ($ arg : ident : $ typ : ty) ,*)) => { define_syscall ! (fn $ name ($ ($ arg : $ typ) ,*) -> ()) ; } }
};
}
