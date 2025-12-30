// Generated macro for impl_18 (impl)
macro_rules! Depcrate_arcsimpl_18 {
() => {
// Module: crate::arcs
// Provides: {"impl_18"}
// Dependencies: {}
impl TryFrom < u8 > for RootArcs { type Error = Error ; # [allow (clippy :: arithmetic_side_effects)] fn try_from (octet : u8) -> Result < Self > { let first = octet as Arc / (ARC_MAX_SECOND + 1) ; let second = octet as Arc % (ARC_MAX_SECOND + 1) ; let result = Self :: new (first , second) ? ; debug_assert_eq ! (octet , result . 0) ; Ok (result) } }
};
}
