// Generated macro for Operation (struct)
macro_rules! Depcrate_astOperation {
() => {
// Module: crate::ast
// Provides: {"Operation"}
// Dependencies: {}
# [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone , Debug , PartialEq)] pub struct Operation < 'a , S > { pub description : Option < Spanning < Cow < 'a , str > > > , pub operation_type : OperationType , pub name : Option < Spanning < & 'a str > > , pub variables_definition : Option < Spanning < VariablesDefinition < 'a , S > > > , pub directives : Option < Vec < Spanning < Directive < 'a , S > > > > , pub selection_set : Vec < Selection < 'a , S > > , }
};
}
