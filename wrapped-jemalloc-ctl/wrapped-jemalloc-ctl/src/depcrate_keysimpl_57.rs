// Generated macro for impl_57 (impl)
macro_rules! Depcrate_keysimpl_57 {
() => {
// Module: crate::keys
// Provides: {"impl_57"}
// Dependencies: {}
impl Access < bool > for Name { fn read (& self) -> Result < bool > { unsafe { let v : u8 = raw :: read (& self . 0) ? ; assert ! (v == 0 || v == 1) ; Ok (v == 1) } } fn write (& self , value : bool) -> Result < () > { unsafe { raw :: write (& self . 0 , value) } } fn update (& self , value : bool) -> Result < bool > { unsafe { let v : u8 = raw :: update (& self . 0 , value as u8) ? ; Ok (v == 1) } } }
};
}
