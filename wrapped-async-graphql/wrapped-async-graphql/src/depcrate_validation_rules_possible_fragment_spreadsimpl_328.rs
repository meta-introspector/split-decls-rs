// Generated macro for impl_328 (impl)
macro_rules! Depcrate_validation_rules_possible_fragment_spreadsimpl_328 {
() => {
// Module: crate::validation::rules::possible_fragment_spreads
// Provides: {"impl_328"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for PossibleFragmentSpreads < 'a > { fn enter_document (& mut self , _ctx : & mut VisitorContext < 'a > , doc : & 'a ExecutableDocument) { for (name , fragment) in doc . fragments . iter () { self . fragment_types . insert (name . as_str () , & fragment . node . type_condition . node . on . node) ; } } fn enter_fragment_spread (& mut self , ctx : & mut VisitorContext < 'a > , fragment_spread : & 'a Positioned < FragmentSpread > ,) { if let Some (fragment_type) = self . fragment_types . get (& * fragment_spread . node . fragment_name . node) { if let Some (current_type) = ctx . current_type () { if let Some (on_type) = ctx . registry . types . get (* fragment_type) { if ! current_type . type_overlap (on_type) { ctx . report_error (vec ! [fragment_spread . pos] , format ! ("Fragment \"{}\" cannot be spread here as objects of type \"{}\" can never be of type \"{}\"" , fragment_spread . node . fragment_name . node , current_type . name () , fragment_type) ,) ; } } } } } fn enter_inline_fragment (& mut self , ctx : & mut VisitorContext < 'a > , inline_fragment : & 'a Positioned < InlineFragment > ,) { if let Some (parent_type) = ctx . parent_type () { if let Some (TypeCondition { on : fragment_type }) = & inline_fragment . node . type_condition . as_ref () . map (| c | & c . node) { if let Some (on_type) = ctx . registry . types . get (fragment_type . node . as_str ()) { if ! parent_type . type_overlap (& on_type) { ctx . report_error (vec ! [inline_fragment . pos] , format ! ("Fragment cannot be spread here as objects of type \"{}\" \
             can never be of type \"{}\"" , parent_type . name () , fragment_type) ,) } } } } } }
};
}
