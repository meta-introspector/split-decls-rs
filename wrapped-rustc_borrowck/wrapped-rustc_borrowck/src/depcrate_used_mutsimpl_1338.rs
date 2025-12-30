// Generated macro for impl_1338 (impl)
macro_rules! Depcrate_used_mutsimpl_1338 {
() => {
// Module: crate::used_muts
// Provides: {"impl_1338"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for GatherUsedMutsVisitor < '_ , '_ , '_ , 'tcx > { fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , location : Location) { debug ! ("visit_terminator: terminator={:?}" , terminator) ; match & terminator . kind { TerminatorKind :: Call { destination , .. } => { self . remove_never_initialized_mut_locals (* destination) ; } _ => { } } self . super_terminator (terminator , location) ; } fn visit_statement (& mut self , statement : & Statement < 'tcx > , location : Location) { if let StatementKind :: Assign (box (into , _)) = & statement . kind { debug ! ("visit_statement: statement={:?} local={:?} \
                    never_initialized_mut_locals={:?}" , statement , into . local , self . never_initialized_mut_locals) ; self . remove_never_initialized_mut_locals (* into) ; } self . super_statement (statement , location) ; } fn visit_local (& mut self , local : Local , place_context : PlaceContext , location : Location) { if place_context . is_place_assignment () && self . temporary_used_locals . contains (& local) { for moi in & self . mbcx . move_data . loc_map [location] { let mpi = & self . mbcx . move_data . moves [* moi] . path ; let path = & self . mbcx . move_data . move_paths [* mpi] ; debug ! ("assignment of {:?} to {:?}, adding {:?} to used mutable set" , path . place , local , path . place) ; if let Some (user_local) = path . place . as_local () { self . mbcx . used_mut . insert (user_local) ; } } } } }
};
}
