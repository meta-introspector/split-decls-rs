// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl bindings :: IClassStatics_Impl for ClassFactory_Impl { fn StaticSignal (& self , value : i32) -> Result < i32 > { let mut counter = 0 ; self . 0 . call (| delegate | { counter += 1 ; delegate . Invoke (self . as_interface () , value) }) ; Ok (counter) } fn StaticEvent (& self , handler : Ref < EventHandler < i32 > >) -> Result < i64 > { self . 0 . add (handler . unwrap ()) } fn RemoveStaticEvent (& self , token : i64) -> Result < () > { self . 0 . remove (token) ; Ok (()) } }
};
}
