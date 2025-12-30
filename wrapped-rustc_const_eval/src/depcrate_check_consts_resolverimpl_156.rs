// Generated macro for impl_156 (impl)
macro_rules! Depcrate_check_consts_resolverimpl_156 {
() => {
// Module: crate::check_consts::resolver
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'tcx , Q > Analysis < 'tcx > for FlowSensitiveAnalysis < '_ , 'tcx , Q > where Q : Qualif , { type Domain = State ; const NAME : & 'static str = Q :: ANALYSIS_NAME ; fn bottom_value (& self , body : & mir :: Body < 'tcx >) -> Self :: Domain { State { qualif : MixedBitSet :: new_empty (body . local_decls . len ()) , borrow : MixedBitSet :: new_empty (body . local_decls . len ()) , } } fn initialize_start_block (& self , _body : & mir :: Body < 'tcx > , state : & mut Self :: Domain) { self . transfer_function (state) . initialize_state () ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , statement : & mir :: Statement < 'tcx > , location : Location ,) { self . transfer_function (state) . visit_statement (statement , location) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { self . transfer_function (state) . visit_terminator (terminator , location) ; terminator . edges () } fn apply_call_return_effect (& mut self , state : & mut Self :: Domain , block : BasicBlock , return_places : CallReturnPlaces < '_ , 'tcx > ,) { self . transfer_function (state) . apply_call_return_effect (block , return_places) } }
};
}
