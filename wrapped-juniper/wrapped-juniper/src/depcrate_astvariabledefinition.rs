// Generated macro for VariableDefinition (struct)
macro_rules! Depcrate_astVariableDefinition {
() => {
// Module: crate::ast
// Provides: {"VariableDefinition"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub struct VariableDefinition < 'a , S > { pub description : Option < Spanning < Cow < 'a , str > > > , pub var_type : Spanning < Type < & 'a str > > , pub default_value : Option < Spanning < InputValue < S > > > , pub directives : Option < Vec < Spanning < Directive < 'a , S > > > > , }
};
}
