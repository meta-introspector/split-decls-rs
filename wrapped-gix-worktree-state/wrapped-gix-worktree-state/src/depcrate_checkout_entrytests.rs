// Generated macro for tests (module)
macro_rules! Depcrate_checkout_entrytests {
() => {
// Module: crate::checkout::entry
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn let_readers_execute () { let cases = [(0o100755 , 0o755) , (0o100644 , 0o755) , (0o100750 , 0o750) , (0o100640 , 0o750) , (0o100700 , 0o700) , (0o100600 , 0o700) , (0o100775 , 0o775) , (0o100664 , 0o775) , (0o100770 , 0o770) , (0o100660 , 0o770) , (0o100764 , 0o775) , (0o100760 , 0o770) , (0o100674 , 0o775) , (0o100670 , 0o770) , (0o100000 , 0o000) , (0o100400 , 0o500) , (0o100440 , 0o550) , (0o100444 , 0o555) , (0o100462 , 0o572) , (0o100242 , 0o252) , (0o100167 , 0o177) , (0o104755 , 0o755) , (0o104644 , 0o755) , (0o102755 , 0o755) , (0o102644 , 0o755) , (0o101755 , 0o755) , (0o101644 , 0o755) , (0o106755 , 0o755) , (0o106644 , 0o755) , (0o104750 , 0o750) , (0o104640 , 0o750) , (0o102750 , 0o750) , (0o102640 , 0o750) , (0o101750 , 0o750) , (0o101640 , 0o750) , (0o106750 , 0o750) , (0o106640 , 0o750) , (0o107644 , 0o755) , (0o107000 , 0o000) , (0o106400 , 0o500) , (0o102462 , 0o572) ,] ; for (st_mode , expected) in cases { let actual = super :: let_readers_execute (st_mode) ; assert_eq ! (actual , expected , "{st_mode:06o} should become {expected:04o}, became {actual:04o}") ; } } # [test] # [should_panic] fn let_readers_execute_panics_on_directory () { super :: let_readers_execute (0o040644) ; } # [test] # [should_panic] fn let_readers_execute_should_panic_on_symlink () { super :: let_readers_execute (0o120644) ; } }
};
}
