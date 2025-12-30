// Generated macro for opaque_strategy_wrapper (macro)
macro_rules! Depcrate_macrosopaque_strategy_wrapper {
() => {
// Module: crate::macros
// Provides: {"opaque_strategy_wrapper"}
// Dependencies: {}
macro_rules ! opaque_strategy_wrapper { ($ ({ # [$ allmeta : meta] }) * $ (# [$ smeta : meta]) * pub struct $ stratname : ident [$ ($ sgen : tt) *] [$ ($ swhere : tt) *] ($ innerstrat : ty) -> $ stratvtty : ty ; $ (# [$ vmeta : meta]) * pub struct $ vtname : ident [$ ($ vgen : tt) *] [$ ($ vwhere : tt) *] ($ innervt : ty) -> $ actualty : ty ;) => { $ (# [$ allmeta]) * $ (# [$ smeta]) * # [must_use = "strategies do nothing unless used"] pub struct $ stratname $ ($ sgen) * ($ innerstrat) $ ($ swhere) *; $ (# [$ allmeta]) * $ (# [$ vmeta]) * pub struct $ vtname $ ($ vgen) * ($ innervt) $ ($ vwhere) *; $ (# [$ allmeta]) * impl $ ($ sgen) * Strategy for $ stratname $ ($ sgen) * $ ($ swhere) * { type Tree = $ stratvtty ; type Value = $ actualty ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . 0 . new_tree (runner) . map ($ vtname) } } $ (# [$ allmeta]) * impl $ ($ vgen) * ValueTree for $ vtname $ ($ vgen) * $ ($ vwhere) * { type Value = $ actualty ; delegate_vt_0 ! () ; } } }
};
}
