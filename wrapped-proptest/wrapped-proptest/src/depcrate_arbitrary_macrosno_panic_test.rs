// Generated macro for no_panic_test (macro)
macro_rules! Depcrate_arbitrary_macrosno_panic_test {
() => {
// Module: crate::arbitrary::macros
// Provides: {"no_panic_test"}
// Dependencies: {}
# [doc = " We are mostly interested in ensuring that generating input from our"] # [doc = " strategies is able to construct a value, therefore ensuring that"] # [doc = " no panic occurs is mostly sufficient. Shrinking for strategies that"] # [doc = " use special shrinking methods can be handled separately."] # [cfg (test)] macro_rules ! no_panic_test { ($ ($ module : ident => $ self : ty) ,+) => { $ (mod $ module { # [allow (unused_imports)] use super :: super ::*; proptest ! { # [test] fn no_panic (_ in $ crate :: arbitrary :: any ::<$ self > ()) { } } }) + } ; }
};
}
