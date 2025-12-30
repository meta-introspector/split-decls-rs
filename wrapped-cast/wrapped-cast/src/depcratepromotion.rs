// Generated macro for promotion (macro)
macro_rules! Depcratepromotion {
() => {
// Module: crate
// Provides: {"promotion"}
// Dependencies: {}
# [doc = " `$dst` can hold any value of `$src`"] macro_rules ! promotion { ($ ($ src : ty => $ ($ dst : ty) ,+) ;+;) => { $ ($ (impl From <$ src > for $ dst { type Output = $ dst ; # [inline] fn cast (src : $ src) -> $ dst { src as $ dst } }) +) + } }
};
}
