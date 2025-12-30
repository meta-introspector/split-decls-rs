// Generated macro for ConstantTimeSelect (trait)
macro_rules! Depcrate_traitsConstantTimeSelect {
() => {
// Module: crate::traits
// Provides: {"ConstantTimeSelect"}
// Dependencies: {}
# [doc = " Trait for types which are conditionally selectable in constant time."] # [doc = ""] # [doc = " Similar to (and blanket impl'd for) `subtle`'s [`ConditionallySelectable`] trait, but without"] # [doc = " the `Copy` bound which allows it to be impl'd for heap allocated types such as `BoxedUint`."] # [doc = ""] # [doc = " It also provides generic implementations of conditional assignment and conditional swaps."] pub trait ConstantTimeSelect : Clone { # [doc = " Select `a` or `b` according to `choice`."] # [doc = ""] # [doc = " # Returns"] # [doc = " - `a` if `choice == Choice(0)`;"] # [doc = " - `b` if `choice == Choice(1)`."] fn ct_select (a : & Self , b : & Self , choice : Choice) -> Self ; # [doc = " Conditionally assign `other` to `self`, according to `choice`."] # [inline] fn ct_assign (& mut self , other : & Self , choice : Choice) { * self = Self :: ct_select (self , other , choice) ; } # [doc = " Conditionally swap `self` and `other` if `choice == 1`; otherwise, reassign both unto themselves."] # [inline] fn ct_swap (a : & mut Self , b : & mut Self , choice : Choice) { let t : Self = a . clone () ; a . ct_assign (b , choice) ; b . ct_assign (& t , choice) ; } }
};
}
