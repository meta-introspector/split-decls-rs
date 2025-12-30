// Generated macro for Fragment (struct)
macro_rules! Depcrate_astFragment {
() => {
// Module: crate::ast
// Provides: {"Fragment"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub struct Fragment < 'a , S > { pub name : Spanning < & 'a str > , pub description : Option < Spanning < Cow < 'a , str > > > , pub type_condition : Spanning < & 'a str > , pub directives : Option < Vec < Spanning < Directive < 'a , S > > > > , pub selection_set : Vec < Selection < 'a , S > > , }
};
}
