// Generated macro for impl_439 (impl)
macro_rules! Depcrate_templateimpl_439 {
() => {
// Module: crate::template
// Provides: {"impl_439"}
// Dependencies: {}
impl Subexpression { pub fn new (name : Parameter , params : Vec < Parameter > , hash : HashMap < String , Parameter > ,) -> Subexpression { Subexpression { element : Box :: new (Expression (Box :: new (HelperTemplate { name , params , hash , template : None , inverse : None , block_param : None , block : false , chain : false , indent_before_write : false , }))) , } } pub fn is_helper (& self) -> bool { match * self . as_element () { TemplateElement :: Expression (ref ht) => ! ht . is_name_only () , _ => false , } } pub fn as_element (& self) -> & TemplateElement { self . element . as_ref () } pub fn name (& self) -> & str { match * self . as_element () { Expression (ref ht) => ht . name . as_name () . unwrap () , _ => unreachable ! () , } } pub fn params (& self) -> Option < & Vec < Parameter > > { match * self . as_element () { Expression (ref ht) => Some (& ht . params) , _ => None , } } pub fn hash (& self) -> Option < & HashMap < String , Parameter > > { match * self . as_element () { Expression (ref ht) => Some (& ht . hash) , _ => None , } } }
};
}
