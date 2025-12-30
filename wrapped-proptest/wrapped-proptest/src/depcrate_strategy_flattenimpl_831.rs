// Generated macro for impl_831 (impl)
macro_rules! Depcrate_strategy_flattenimpl_831 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_831"}
// Dependencies: {}
impl < S : ValueTree > ValueTree for FlattenValueTree < S > where S :: Value : Strategy , { type Value = < S :: Value as Strategy > :: Value ; fn current (& self) -> Self :: Value { self . current . current () } fn simplify (& mut self) -> bool { self . complicate_regen_remaining = 0 ; if self . current . simplify () { self . meta . disallow_complicate () ; self . final_complication = None ; true } else if ! self . meta . simplify () { false } else if let Ok (v) = self . meta . current () . new_tree (& mut self . runner) { self . current . disallow_complicate () ; self . final_complication = Some (Fuse :: new (v)) ; mem :: swap (self . final_complication . as_mut () . unwrap () , & mut self . current ,) ; self . complicate_regen_remaining = self . runner . config () . cases ; true } else { false } } fn complicate (& mut self) -> bool { if self . complicate_regen_remaining > 0 { if self . runner . flat_map_regen () { self . complicate_regen_remaining -= 1 ; if let Ok (v) = self . meta . current () . new_tree (& mut self . runner) { self . current = Fuse :: new (v) ; return true ; } } else { self . complicate_regen_remaining = 0 ; } } if self . current . complicate () { return true ; } else if self . meta . complicate () { if let Ok (v) = self . meta . current () . new_tree (& mut self . runner) { self . complicate_regen_remaining = self . runner . config () . cases ; self . current = Fuse :: new (v) ; return true ; } } if let Some (v) = self . final_complication . take () { self . current = v ; true } else { false } } }
};
}
