// Generated macro for impl_59 (impl)
macro_rules! Depcrate_keysimpl_59 {
() => {
// Module: crate::keys
// Provides: {"impl_59"}
// Dependencies: {}
impl Access < & 'static [u8] > for Name { fn read (& self) -> Result < & 'static [u8] > { assert ! (self . value_type_str () , "the name \"{:?}\" does not refer to a byte string" , self) ; unsafe { raw :: read_str (& self . 0) } } fn write (& self , value : & 'static [u8]) -> Result < () > { assert ! (self . value_type_str () , "the name \"{:?}\" does not refer to a byte string" , self) ; raw :: write_str (& self . 0 , value) } fn update (& self , value : & 'static [u8]) -> Result < & 'static [u8] > { assert ! (self . value_type_str () , "the name \"{:?}\" does not refer to a byte string" , self) ; unsafe { raw :: update_str (& self . 0 , value) } } }
};
}
