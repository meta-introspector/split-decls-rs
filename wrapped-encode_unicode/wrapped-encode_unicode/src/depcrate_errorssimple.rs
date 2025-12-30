// Generated macro for simple (macro)
macro_rules! Depcrate_errorssimple {
() => {
// Module: crate::errors
// Provides: {"simple"}
// Dependencies: {}
macro_rules ! simple { ($ (# [$ tydoc : meta]) * $ err : ident { $ ($ (# [$ vardoc : meta]) * $ variant : ident => $ string : expr ,) + }) => { $ (# [$ tydoc]) * # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum $ err { $ ($ (# [$ vardoc]) * $ variant ,) * } description ! { $ err , | e : &$ err | match * e { $ ($ err ::$ variant => $ string) ,* } } } }
};
}
