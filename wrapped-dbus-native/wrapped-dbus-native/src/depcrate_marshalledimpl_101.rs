// Generated macro for impl_101 (impl)
macro_rules! Depcrate_marshalledimpl_101 {
() => {
// Module: crate::marshalled
// Provides: {"impl_101"}
// Dependencies: {}
impl Parsed < '_ > { pub fn as_dbus_str (& self) -> Result < & DBusStr , DemarshalError > { match self { Parsed :: String (x) => Ok (x) , Parsed :: ObjectPath (x) => Ok (x . as_dbus_str ()) , Parsed :: Signature (x) => Ok (x . as_dbus_str ()) , _ => Err (DemarshalError :: WrongType) , } } }
};
}
