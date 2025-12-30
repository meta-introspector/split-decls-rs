// Generated macro for impl_39 (impl)
macro_rules! Depcrate_com_objectimpl_39 {
() => {
// Module: crate::com_object
// Provides: {"impl_39"}
// Dependencies: {}
impl MyApp { fn new (x : u32) -> ComObject < Self > { ComObject :: new (Self { x , signature : APP_SIGNATURE , tombstone : Arc :: new (Tombstone :: default ()) , }) } fn get_x_direct (& self) -> u32 { self . x } fn set_x (& mut self , x : u32) { self . x = x ; } }
};
}
