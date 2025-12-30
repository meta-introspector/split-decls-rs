// Generated macro for impl_61 (impl)
macro_rules! Depcrate_keysimpl_61 {
() => {
// Module: crate::keys
// Provides: {"impl_61"}
// Dependencies: {}
impl Access < & 'static str > for Name { fn read (& self) -> Result < & 'static str > { assert ! (self . value_type_str () , "the name \"{:?}\" does not refer to a byte string" , self) ; let s = unsafe { raw :: read_str (& self . 0) ? } ; Ok (str :: from_utf8 (s) . unwrap ()) } fn write (& self , value : & 'static str) -> Result < () > { assert ! (self . value_type_str () , "the name \"{:?}\" does not refer to a byte string" , self) ; raw :: write_str (& self . 0 , value . as_bytes ()) } fn update (& self , value : & 'static str) -> Result < & 'static str > { assert ! (self . value_type_str () , "the name \"{:?}\" does not refer to a byte string" , self) ; let s = unsafe { raw :: update_str (& self . 0 , value . as_bytes ()) ? } ; Ok (str :: from_utf8 (s) . unwrap ()) } }
};
}
