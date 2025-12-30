// Generated macro for from_signed (macro)
macro_rules! Depcratefrom_signed {
() => {
// Module: crate
// Provides: {"from_signed"}
// Dependencies: {}
# [doc = " From a signed `$src` to a smaller `$dst`"] macro_rules ! from_signed { ($ ($ src : ident => $ ($ dst : ident) ,+) ;+;) => { $ ($ (impl From <$ src > for $ dst { type Output = Result <$ dst , Error >; # [inline] fn cast (src : $ src) -> Self :: Output { use core ::$ dst ; Err (if src < $ dst :: MIN as $ src { Error :: Underflow } else if src > $ dst :: MAX as $ src { Error :: Overflow } else { return Ok (src as $ dst) ; }) } }) +) + } }
};
}
