// Generated macro for in_selector_family (function)
macro_rules! Depcrate_objc2_utilsin_selector_family {
() => {
// Module: crate::objc2_utils
// Provides: {"in_selector_family"}
// Dependencies: {}
pub const fn in_selector_family (mut selector : & [u8] , mut family : & [u8]) -> bool { loop { selector = match selector { [b'_' , rest @ ..] => rest , _ => break , } } loop { (selector , family) = match (selector , family) { ([s , selector @ ..] , [f , family @ ..]) => { if * s == * f { (selector , family) } else { return false ; } } ([] , []) => { return true ; } ([] , _) => { return false ; } ([s , ..] , []) => { return ! s . is_ascii_lowercase () ; } } } }
};
}
