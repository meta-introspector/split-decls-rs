// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl Deref for MmapMut { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . inner . ptr () , self . inner . len ()) } } }
};
}
