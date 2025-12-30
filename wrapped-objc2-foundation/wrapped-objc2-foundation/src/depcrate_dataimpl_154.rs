// Generated macro for impl_154 (impl)
macro_rules! Depcrate_dataimpl_154 {
() => {
// Module: crate::data
// Provides: {"impl_154"}
// Dependencies: {}
impl NSData { pub fn len (& self) -> usize { self . length () } pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " The bytes in the data."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The data must not be mutated while the returned slice is alive."] # [doc = " Consider using [`to_vec`] instead if this requirement is a bit"] # [doc = " difficult to uphold."] # [doc = ""] # [doc = " [`to_vec`]: Self::to_vec"] pub unsafe fn as_bytes_unchecked (& self) -> & [u8] { let ptr = self . bytes_raw () ; if ! ptr . is_null () { let ptr : * const u8 = ptr . cast () ; unsafe { slice :: from_raw_parts (ptr , self . len ()) } } else { & [] } } # [doc = " Copy the contents of the data into a new [`Vec`]."] pub fn to_vec (& self) -> Vec < u8 > { let mut vec = Vec :: with_capacity (self . len ()) ; vec . extend_from_slice (unsafe { self . as_bytes_unchecked () }) ; vec } # [doc = " Iterate over the bytes of the data."] pub fn iter (& self) -> Iter < '_ > { Iter :: new (self) } }
};
}
