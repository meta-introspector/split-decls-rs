// Generated macro for half_promotion (macro)
macro_rules! Depcratehalf_promotion {
() => {
// Module: crate
// Provides: {"half_promotion"}
// Dependencies: {}
# [doc = " `$dst` can hold any positive value of `$src`"] macro_rules ! half_promotion { ($ ($ src : ty => $ ($ dst : ty) ,+) ;+;) => { $ ($ (impl From <$ src > for $ dst { type Output = Result <$ dst , Error >; # [inline] fn cast (src : $ src) -> Self :: Output { if src < 0 { Err (Error :: Underflow) } else { Ok (src as $ dst) } } }) +) + } }
};
}
