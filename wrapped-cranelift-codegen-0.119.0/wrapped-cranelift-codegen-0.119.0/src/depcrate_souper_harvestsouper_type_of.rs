// Generated macro for souper_type_of (function)
macro_rules! Depcrate_souper_harvestsouper_type_of {
() => {
// Module: crate::souper_harvest
// Provides: {"souper_type_of"}
// Dependencies: {}
fn souper_type_of (dfg : & ir :: DataFlowGraph , val : ir :: Value) -> Option < ast :: Type > { let ty = dfg . value_type (val) ; assert ! (ty . is_int ()) ; assert_eq ! (ty . lane_count () , 1) ; let width = match dfg . value_def (val) . inst () { Some (inst) if dfg . insts [inst] . opcode () == ir :: Opcode :: IcmpImm || dfg . insts [inst] . opcode () == ir :: Opcode :: Icmp => { 1 } _ => ty . bits () . try_into () . unwrap () , } ; Some (ast :: Type { width }) }
};
}
