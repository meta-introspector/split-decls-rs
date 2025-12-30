// Generated macro for allow_duplicates (macro)
macro_rules! Depcrate_macrosallow_duplicates {
() => {
// Module: crate::macros
// Provides: {"allow_duplicates"}
// Dependencies: {}
# [doc = " Utility macro to permit a multi-snapshot run where all snapshots match."] # [doc = ""] # [doc = " Within this block, insta will allow an assertion to be run more than once"] # [doc = " (even inline) without generating another snapshot.  Instead it will assert"] # [doc = " that snapshot expressions visited more than once are matching."] # [doc = ""] # [doc = " ```rust"] # [doc = " insta::allow_duplicates! {"] # [doc = "     for x in (0..10).step_by(2) {"] # [doc = "         let is_even = x % 2 == 0;"] # [doc = "         insta::assert_debug_snapshot!(is_even, @\"true\");"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The first snapshot assertion will be used as a gold master and every further"] # [doc = " assertion will be checked against it.  If they don't match the assertion will"] # [doc = " fail."] # [macro_export] macro_rules ! allow_duplicates { ($ ($ x : tt) *) => { $ crate :: _macro_support :: with_allow_duplicates (|| { $ ($ x) * }) } }
};
}
