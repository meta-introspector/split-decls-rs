// Generated macro for InputValue (struct)
macro_rules! Depcrate_dynamic_input_valueInputValue {
() => {
// Module: crate::dynamic::input_value
// Provides: {"InputValue"}
// Dependencies: {}
# [doc = " A GraphQL input value type"] # [derive (Debug)] pub struct InputValue { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) ty : TypeRef , pub (crate) default_value : Option < Value > , pub (crate) inaccessible : bool , pub (crate) tags : Vec < String > , pub (crate) directives : Vec < Directive > , pub (crate) deprecation : Deprecation , }
};
}
