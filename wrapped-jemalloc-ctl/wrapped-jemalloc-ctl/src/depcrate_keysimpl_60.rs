// Generated macro for impl_60 (impl)
macro_rules! Depcrate_keysimpl_60 {
() => {
// Module: crate::keys
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : MibArg > Access < & 'static str > for MibStr < T > { fn read (& self) -> Result < & 'static str > { let s = unsafe { raw :: read_str_mib (self . 0 . as_ref ()) ? } ; Ok (str :: from_utf8 (s) . unwrap ()) } fn write (& self , value : & 'static str) -> Result < () > { raw :: write_str_mib (self . 0 . as_ref () , value . as_bytes ()) } fn update (& self , value : & 'static str) -> Result < & 'static str > { let s = unsafe { raw :: update_str_mib (self . 0 . as_ref () , value . as_bytes ()) ? } ; Ok (str :: from_utf8 (s) . unwrap ()) } }
};
}
