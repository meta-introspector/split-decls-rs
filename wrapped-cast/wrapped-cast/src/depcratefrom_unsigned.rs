// Generated macro for from_unsigned (macro)
macro_rules! Depcratefrom_unsigned {
() => {
// Module: crate
// Provides: {"from_unsigned"}
// Dependencies: {}
# [doc = " From an unsigned `$src` to a smaller `$dst`"] macro_rules ! from_unsigned { ($ ($ src : ident => $ ($ dst : ident) ,+) ;+;) => { $ ($ (impl From <$ src > for $ dst { type Output = Result <$ dst , Error >; # [inline] fn cast (src : $ src) -> Self :: Output { use core ::$ dst ; if src > $ dst :: MAX as $ src { Err (Error :: Overflow) } else { Ok (src as $ dst) } } }) +) + } }
};
}
