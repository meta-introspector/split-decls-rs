// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl bindings :: IClass_Impl for Class_Impl { fn Signal (& self , value : i32) -> Result < i32 > { let mut counter = 0 ; self . 0 . call (| delegate | { counter += 1 ; delegate . Invoke (self . as_interface () , value) }) ; Ok (counter) } fn Event (& self , handler : Ref < TypedEventHandler < bindings :: Class , i32 > > ,) -> windows_core :: Result < i64 > { self . 0 . add (handler . unwrap ()) } fn RemoveEvent (& self , token : i64) -> windows_core :: Result < () > { self . 0 . remove (token) ; Ok (()) } }
};
}
