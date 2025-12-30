// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl Arguments { # [doc = " Returns `true` if the given test should be ignored."] fn is_ignored (& self , test : & Trial) -> bool { (test . info . is_ignored && ! self . ignored && ! self . include_ignored) || (test . info . is_bench && self . test) || (! test . info . is_bench && self . bench) } fn is_filtered_out (& self , test : & Trial) -> bool { let test_name = test . name () ; let test_name_with_kind = test . info . test_name_with_kind () ; if let Some (filter) = & self . filter { match self . exact { true if test_name != filter && & test_name_with_kind != filter => return true , false if ! test_name_with_kind . contains (filter) => return true , _ => { } } ; } for skip_filter in & self . skip { match self . exact { true if test_name == skip_filter || & test_name_with_kind == skip_filter => { return true } false if test_name_with_kind . contains (skip_filter) => return true , _ => { } } } if self . ignored && ! test . info . is_ignored { return true ; } false } }
};
}
