// Generated macro for impl_access (macro)
macro_rules! Depcrate_keysimpl_access {
() => {
// Module: crate::keys
// Provides: {"impl_access"}
// Dependencies: {}
macro_rules ! impl_access { ($ id : ty) => { impl < T : MibArg > Access <$ id > for Mib < T > { fn read (& self) -> Result <$ id > { unsafe { raw :: read_mib (self . 0 . as_ref ()) } } fn write (& self , value : $ id) -> Result < () > { unsafe { raw :: write_mib (self . 0 . as_ref () , value) } } fn update (& self , value : $ id) -> Result <$ id > { unsafe { raw :: update_mib (self . 0 . as_ref () , value) } } } impl Access <$ id > for Name { fn read (& self) -> Result <$ id > { unsafe { raw :: read (& self . 0) } } fn write (& self , value : $ id) -> Result < () > { unsafe { raw :: write (& self . 0 , value) } } fn update (& self , value : $ id) -> Result <$ id > { unsafe { raw :: update (& self . 0 , value) } } } } ; }
};
}
