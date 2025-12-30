// Generated macro for promote_and_back (macro)
macro_rules! Depcrate_testpromote_and_back {
() => {
// Module: crate::test
// Provides: {"promote_and_back"}
// Dependencies: {}
macro_rules ! promote_and_back { ($ ($ src : ident => $ ($ dst : ident) ,+) ;+;) => { mod demoting_to { $ (mod $ src { mod from { use crate :: From ; $ (quickcheck ! { fn $ dst (src : $ src) -> bool { $ src :: cast ($ dst :: cast (src)) . is_ok () } }) + } }) + } } }
};
}
