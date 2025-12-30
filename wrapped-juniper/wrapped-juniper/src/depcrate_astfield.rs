// Generated macro for Field (struct)
macro_rules! Depcrate_astField {
() => {
// Module: crate::ast
// Provides: {"Field"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub struct Field < 'a , S > { pub alias : Option < Spanning < & 'a str > > , pub name : Spanning < & 'a str > , pub arguments : Option < Spanning < Arguments < 'a , S > > > , pub directives : Option < Vec < Spanning < Directive < 'a , S > > > > , pub selection_set : Option < Vec < Selection < 'a , S > > > , }
};
}
