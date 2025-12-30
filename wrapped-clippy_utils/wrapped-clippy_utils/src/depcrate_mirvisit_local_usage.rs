// Generated macro for visit_local_usage (function)
macro_rules! Depcrate_mirvisit_local_usage {
() => {
// Module: crate::mir
// Provides: {"visit_local_usage"}
// Dependencies: {}
pub fn visit_local_usage (locals : & [Local] , mir : & Body < '_ > , location : Location) -> Option < Vec < LocalUsage > > { let init = vec ! [LocalUsage { local_use_locs : Vec :: new () , local_consume_or_mutate_locs : Vec :: new () , } ; locals . len ()] ; traversal :: Postorder :: new (& mir . basic_blocks , location . block , None) . collect :: < Vec < _ > > () . into_iter () . rev () . try_fold (init , | usage , tbb | { let tdata = & mir . basic_blocks [tbb] ; if tdata . terminator () . successors () . any (| s | s == location . block) { return None ; } let mut v = V { locals , location , results : usage , } ; v . visit_basic_block_data (tbb , tdata) ; Some (v . results) }) }
};
}
