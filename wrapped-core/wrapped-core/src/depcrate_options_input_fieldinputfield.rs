// Generated macro for InputField (struct)
macro_rules! Depcrate_options_input_fieldInputField {
() => {
// Module: crate::options::input_field
// Provides: {"InputField"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct InputField { pub ident : syn :: Ident , pub attr_name : Option < String > , pub ty : syn :: Type , pub default : Option < DefaultExpression > , pub with : Option < Callable > , # [doc = " If `true`, generated code will not look for this field in the input meta item,"] # [doc = " instead always falling back to either `InputField::default` or `Default::default`."] pub skip : Option < SpannedValue < bool > > , pub post_transform : Option < codegen :: PostfixTransform > , pub multiple : Option < bool > , pub flatten : Flag , }
};
}
