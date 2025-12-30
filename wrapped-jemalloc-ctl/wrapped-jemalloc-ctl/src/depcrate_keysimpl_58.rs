// Generated macro for impl_58 (impl)
macro_rules! Depcrate_keysimpl_58 {
() => {
// Module: crate::keys
// Provides: {"impl_58"}
// Dependencies: {}
impl < T : MibArg > Access < & 'static [u8] > for MibStr < T > { fn read (& self) -> Result < & 'static [u8] > { unsafe { raw :: read_str_mib (self . 0 . as_ref ()) } } fn write (& self , value : & 'static [u8]) -> Result < () > { raw :: write_str_mib (self . 0 . as_ref () , value) } fn update (& self , value : & 'static [u8]) -> Result < & 'static [u8] > { unsafe { raw :: update_str_mib (self . 0 . as_ref () , value) } } }
};
}
