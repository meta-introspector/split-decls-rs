// Generated macro for lock (macro)
macro_rules! Depcrate_os_iocplock {
() => {
// Module: crate::os::iocp
// Provides: {"lock"}
// Dependencies: {}
# [doc = " Macro to lock and ignore lock poisoning."] macro_rules ! lock { ($ lock_result : expr) => { { $ lock_result . unwrap_or_else (| e | e . into_inner ()) } } ; }
};
}
