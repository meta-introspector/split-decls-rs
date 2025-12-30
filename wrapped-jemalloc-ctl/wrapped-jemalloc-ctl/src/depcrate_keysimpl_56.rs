// Generated macro for impl_56 (impl)
macro_rules! Depcrate_keysimpl_56 {
() => {
// Module: crate::keys
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : MibArg > Access < bool > for Mib < T > { fn read (& self) -> Result < bool > { unsafe { let v : u8 = raw :: read_mib (self . 0 . as_ref ()) ? ; assert ! (v == 0 || v == 1) ; Ok (v == 1) } } fn write (& self , value : bool) -> Result < () > { unsafe { raw :: write_mib (self . 0 . as_ref () , value) } } fn update (& self , value : bool) -> Result < bool > { unsafe { let v : u8 = raw :: update_mib (self . 0 . as_ref () , value as u8) ? ; Ok (v == 1) } } }
};
}
