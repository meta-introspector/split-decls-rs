// Generated macro for impl_306 (impl)
macro_rules! Depcrate_validation_rules_no_unused_fragmentsimpl_306 {
() => {
// Module: crate::validation::rules::no_unused_fragments
// Provides: {"impl_306"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for NoUnusedFragments < 'a > { fn exit_document (& mut self , ctx : & mut VisitorContext < 'a > , doc : & 'a ExecutableDocument) { let mut reachable = HashSet :: new () ; for (name , _) in doc . operations . iter () { self . find_reachable_fragments (& Scope :: Operation (name . map (Name :: as_str)) , & mut reachable ,) ; } for (fragment_name , pos) in & self . defined_fragments { if ! reachable . contains (fragment_name) { ctx . report_error (vec ! [* pos] , format ! (r#"Fragment "{}" is never used"# , fragment_name) ,) ; } } } fn enter_operation_definition (& mut self , _ctx : & mut VisitorContext < 'a > , name : Option < & 'a Name > , _operation_definition : & 'a Positioned < OperationDefinition > ,) { self . current_scope = Some (Scope :: Operation (name . map (Name :: as_str))) ; } fn enter_fragment_definition (& mut self , _ctx : & mut VisitorContext < 'a > , name : & 'a Name , fragment_definition : & 'a Positioned < FragmentDefinition > ,) { self . current_scope = Some (Scope :: Fragment (name)) ; self . defined_fragments . insert ((name , fragment_definition . pos)) ; } fn enter_fragment_spread (& mut self , _ctx : & mut VisitorContext < 'a > , fragment_spread : & 'a Positioned < FragmentSpread > ,) { if let Some (ref scope) = self . current_scope { self . spreads . entry (* scope) . or_default () . push (& fragment_spread . node . fragment_name . node) ; } } }
};
}
