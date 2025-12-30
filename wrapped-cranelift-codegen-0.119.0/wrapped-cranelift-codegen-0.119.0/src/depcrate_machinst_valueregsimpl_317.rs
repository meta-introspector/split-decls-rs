// Generated macro for impl_317 (impl)
macro_rules! Depcrate_machinst_valueregsimpl_317 {
() => {
// Module: crate::machinst::valueregs
// Provides: {"impl_317"}
// Dependencies: {}
impl < R : Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel > Debug for ValueRegs < R > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut f = f . debug_tuple ("ValueRegs") ; let mut last_valid = true ; for part in self . parts { if part . is_invalid_sentinel () { last_valid = false ; } else { debug_assert ! (last_valid) ; f . field (& part) ; } } f . finish () } }
};
}
