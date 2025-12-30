// Generated macro for One (trait)
macro_rules! Depcrate_identitiesOne {
() => {
// Module: crate::identities
// Provides: {"One"}
// Dependencies: {}
# [doc = " Defines a multiplicative identity element for `Self`."] # [doc = ""] # [doc = " # Laws"] # [doc = ""] # [doc = " ```text"] # [doc = " a * 1 = a       ∀ a ∈ Self"] # [doc = " 1 * a = a       ∀ a ∈ Self"] # [doc = " ```"] pub trait One : Sized + Mul < Self , Output = Self > { # [doc = " Returns the multiplicative identity element of `Self`, `1`."] # [doc = ""] # [doc = " # Purity"] # [doc = ""] # [doc = " This function should return the same result at all times regardless of"] # [doc = " external mutable state, for example values stored in TLS or in"] # [doc = " `static mut`s."] fn one () -> Self ; # [doc = " Sets `self` to the multiplicative identity element of `Self`, `1`."] fn set_one (& mut self) { * self = One :: one () ; } # [doc = " Returns `true` if `self` is equal to the multiplicative identity."] # [doc = ""] # [doc = " For performance reasons, it's best to implement this manually."] # [doc = " After a semver bump, this method will be required, and the"] # [doc = " `where Self: PartialEq` bound will be removed."] # [inline] fn is_one (& self) -> bool where Self : PartialEq , { * self == Self :: one () } }
};
}
