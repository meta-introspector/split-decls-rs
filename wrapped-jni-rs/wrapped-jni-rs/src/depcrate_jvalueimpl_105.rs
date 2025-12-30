// Generated macro for impl_105 (impl)
macro_rules! Depcrate_jvalueimpl_105 {
() => {
// Module: crate::jvalue
// Provides: {"impl_105"}
// Dependencies: {}
impl TryFrom < JValue < '_ > > for jboolean { type Error = Error ; fn try_from (value : JValue) -> Result < Self > { match value { JValue :: Bool (b) => Ok (b) , _ => Err (Error :: WrongJValueType ("bool" , value . type_name ())) , } } }
};
}
