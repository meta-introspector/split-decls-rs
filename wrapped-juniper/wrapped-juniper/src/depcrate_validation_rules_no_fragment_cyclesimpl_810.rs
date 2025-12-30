// Generated macro for impl_810 (impl)
macro_rules! Depcrate_validation_rules_no_fragment_cyclesimpl_810 {
() => {
// Module: crate::validation::rules::no_fragment_cycles
// Provides: {"impl_810"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for NoFragmentCycles < 'a > where S : ScalarValue , { fn exit_document (& mut self , ctx : & mut ValidatorContext < 'a , S > , _ : & 'a Document < S >) { assert ! (self . current_fragment . is_none ()) ; let mut detector = CycleDetector { visited : HashSet :: new () , spreads : & self . spreads , errors : Vec :: new () , } ; for frag in & self . fragment_order { if ! detector . visited . contains (frag) { detector . detect_from (frag) ; } } ctx . append_errors (detector . errors) ; } fn enter_fragment_definition (& mut self , _ : & mut ValidatorContext < 'a , S > , fragment : & 'a Spanning < Fragment < S > > ,) { assert ! (self . current_fragment . is_none ()) ; let fragment_name = & fragment . item . name . item ; self . current_fragment = Some (fragment_name) ; self . fragment_order . push (fragment_name) ; } fn exit_fragment_definition (& mut self , _ : & mut ValidatorContext < 'a , S > , fragment : & 'a Spanning < Fragment < S > > ,) { assert_eq ! (Some (fragment . item . name . item) , self . current_fragment) ; self . current_fragment = None ; } fn enter_fragment_spread (& mut self , _ : & mut ValidatorContext < 'a , S > , spread : & 'a Spanning < FragmentSpread < S > > ,) { if let Some (current_fragment) = self . current_fragment { self . spreads . entry (current_fragment) . or_default () . push (BorrowedSpanning { item : spread . item . name . item , span : & spread . span , }) ; } } }
};
}
