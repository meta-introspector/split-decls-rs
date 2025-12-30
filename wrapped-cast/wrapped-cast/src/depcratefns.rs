// Generated macro for fns (macro)
macro_rules! Depcratefns {
() => {
// Module: crate
// Provides: {"fns"}
// Dependencies: {}
macro_rules ! fns { ($ ($ ty : ident) ,+) => { $ (# [doc = " Checked cast function"] # [inline] pub fn $ ty < T > (x : T) -> <$ ty as From < T >>:: Output where $ ty : From < T > { <$ ty as From < T >>:: cast (x) }) + } }
};
}
