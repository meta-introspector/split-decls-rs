// Generated macro for in_selector_family (function)
macro_rules! Depcrate___macros_method_familyin_selector_family {
() => {
// Module: crate::__macros::method_family
// Provides: {"in_selector_family"}
// Dependencies: {}
# [doc = " Checks whether a given selector is said to be in a given selector family."] # [doc = ""] # [doc = " <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families>"] const fn in_selector_family (mut selector : & [u8] , mut family : & [u8]) -> bool { loop { selector = match selector { [b'_' , rest @ ..] => rest , _ => break , } } loop { (selector , family) = match (selector , family) { ([s , selector @ ..] , [f , family @ ..]) => { if * s == * f { (selector , family) } else { return false ; } } ([] , []) => { return true ; } ([] , _) => { return false ; } ([s , ..] , []) => { return ! s . is_ascii_lowercase () ; } } } }
};
}
