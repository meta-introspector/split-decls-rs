// Generated macro for WrongInputScalarTypeError (struct)
macro_rules! Depcrate_value_scalarWrongInputScalarTypeError {
() => {
// Module: crate::value::scalar
// Provides: {"WrongInputScalarTypeError"}
// Dependencies: {}
# [doc = " Error of a [`ScalarValue`] not matching the expected type."] # [derive (Clone , Debug , Display , Error)] # [display ("Expected `{type_name}`, found: {}" , <& Scalar < _ >>:: from (* input))] pub struct WrongInputScalarTypeError < 'a , S : ScalarValue > { # [doc = " Type name of the expected GraphQL scalar."] pub type_name : ArcStr , # [doc = " Input [`ScalarValue`] not matching the expected type."] pub input : & 'a S , }
};
}
