// Generated macro for impl_relative_eq (macro)
macro_rules! Depcrate_relative_eqimpl_relative_eq {
() => {
// Module: crate::relative_eq
// Provides: {"impl_relative_eq"}
// Dependencies: {}
macro_rules ! impl_relative_eq { ($ T : ident , $ U : ident) => { impl RelativeEq for $ T { # [inline] fn default_max_relative () -> $ T { $ T :: EPSILON } # [inline] # [allow (unused_imports)] fn relative_eq (& self , other : &$ T , epsilon : $ T , max_relative : $ T) -> bool { use num_traits :: float :: FloatCore ; if self == other { return true ; } if $ T :: is_infinite (* self) || $ T :: is_infinite (* other) { return false ; } let abs_diff = $ T :: abs (self - other) ; if abs_diff <= epsilon { return true ; } let abs_self = $ T :: abs (* self) ; let abs_other = $ T :: abs (* other) ; let largest = if abs_other > abs_self { abs_other } else { abs_self } ; abs_diff <= largest * max_relative } } } ; }
};
}
