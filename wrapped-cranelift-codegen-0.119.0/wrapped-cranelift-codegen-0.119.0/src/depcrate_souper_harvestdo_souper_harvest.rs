// Generated macro for do_souper_harvest (function)
macro_rules! Depcrate_souper_harvestdo_souper_harvest {
() => {
// Module: crate::souper_harvest
// Provides: {"do_souper_harvest"}
// Dependencies: {}
# [doc = " Harvest Souper left-hand side candidates from the given function."] # [doc = ""] # [doc = " Candidates are reported through the given MPSC sender."] pub fn do_souper_harvest (func : & ir :: Function , out : & mut mpsc :: Sender < String >) { let mut allocs = Allocs :: default () ; for block in func . layout . blocks () { let mut option_inst = func . layout . first_inst (block) ; while let Some (inst) = option_inst { let results = func . dfg . inst_results (inst) ; if results . len () == 1 { let val = results [0] ; let ty = func . dfg . value_type (val) ; if ty . is_int () && ty . lane_count () == 1 { harvest_candidate_lhs (& mut allocs , func , val , out) ; } } option_inst = func . layout . next_inst (inst) ; } } }
};
}
