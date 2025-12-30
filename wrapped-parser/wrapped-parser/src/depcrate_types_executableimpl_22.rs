// Generated macro for impl_22 (impl)
macro_rules! Depcrate_types_executableimpl_22 {
() => {
// Module: crate::types::executable
// Provides: {"impl_22"}
// Dependencies: {}
impl VariableDefinition { # [doc = " Get the default value of the variable; this is `default_value` if it is"] # [doc = " present, `Value::Null` if it is nullable and `None` otherwise."] # [must_use] pub fn default_value (& self) -> Option < & ConstValue > { self . default_value . as_ref () . map (| value | & value . node) . or ({ if self . var_type . node . nullable { Some (& ConstValue :: Null) } else { None } }) } }
};
}
