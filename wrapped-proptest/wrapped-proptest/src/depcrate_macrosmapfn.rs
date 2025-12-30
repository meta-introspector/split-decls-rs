// Generated macro for mapfn (macro)
macro_rules! Depcrate_macrosmapfn {
() => {
// Module: crate::macros
// Provides: {"mapfn"}
// Dependencies: {}
macro_rules ! mapfn { ($ ({ # [$ allmeta : meta] }) * $ (# [$ meta : meta]) * [$ ($ vis : tt) *] fn $ name : ident [$ ($ gen : tt) *] ($ parm : ident : $ input : ty) -> $ output : ty { $ ($ body : tt) * }) => { $ (# [$ allmeta]) * $ (# [$ meta]) * # [derive (Clone , Copy , Debug)] $ ($ vis) * struct $ name ; $ (# [$ allmeta]) * impl $ ($ gen) * $ crate :: strategy :: statics :: MapFn <$ input > for $ name { type Output = $ output ; fn apply (& self , $ parm : $ input) -> $ output { $ ($ body) * } } } }
};
}
