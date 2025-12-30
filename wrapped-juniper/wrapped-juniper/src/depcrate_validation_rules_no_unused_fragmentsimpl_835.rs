// Generated macro for impl_835 (impl)
macro_rules! Depcrate_validation_rules_no_unused_fragmentsimpl_835 {
() => {
// Module: crate::validation::rules::no_unused_fragments
// Provides: {"impl_835"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for NoUnusedFragments < 'a > where S : ScalarValue , { fn exit_document (& mut self , ctx : & mut ValidatorContext < 'a , S > , defs : & 'a Document < S >) { let mut reachable = HashSet :: new () ; for def in defs { if let Definition :: Operation (Spanning { item : Operation { ref name , .. } , .. }) = * def { let op_name = name . as_ref () . map (| s | s . item) ; self . find_reachable_fragments (Scope :: Operation (op_name) , & mut reachable) ; } } for fragment in & self . defined_fragments { if ! reachable . contains (& fragment . item) { ctx . report_error (& error_message (fragment . item) , & [fragment . span . start]) ; } } } fn enter_operation_definition (& mut self , _ : & mut ValidatorContext < 'a , S > , op : & 'a Spanning < Operation < S > > ,) { let op_name = op . item . name . as_ref () . map (| s | s . item) ; self . current_scope = Some (Scope :: Operation (op_name)) ; } fn enter_fragment_definition (& mut self , _ : & mut ValidatorContext < 'a , S > , f : & 'a Spanning < Fragment < S > > ,) { self . current_scope = Some (Scope :: Fragment (f . item . name . item)) ; self . defined_fragments . insert (BorrowedSpanning { span : & f . span , item : f . item . name . item , }) ; } fn enter_fragment_spread (& mut self , _ : & mut ValidatorContext < 'a , S > , spread : & 'a Spanning < FragmentSpread < S > > ,) { if let Some (ref scope) = self . current_scope { self . spreads . entry (* scope) . or_default () . push (spread . item . name . item) ; } } }
};
}
