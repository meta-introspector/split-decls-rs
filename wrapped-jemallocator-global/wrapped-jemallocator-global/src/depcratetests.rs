// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { macro_rules ! check { () => { # [test] fn foo () { let _ = super :: JEMALLOC ; } } ; ($ os_name : tt) => { # [cfg (target_os = $ os_name)] check ! () ; } ; ($ ($ os_name : tt) ,*) => { $ (check ! ($ os_name) ;) * } } # [cfg (feature = "force_global_jemalloc")] check ! () ; # [cfg (not (feature = "force_global_jemalloc"))] check ! ("linux" , "android" , "macos" , "ios" , "freebsd" , "netbsd" , "openbsd") ; }
};
}
