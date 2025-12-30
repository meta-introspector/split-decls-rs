// Generated macro for impl_3045 (impl)
macro_rules! Depcrate_optsimpl_3045 {
() => {
// Module: crate::opts
// Provides: {"impl_3045"}
// Dependencies: {}
impl < 'a , 'b , 'c > ContextIter for InstDataEtorIter < 'a , 'b , 'c > where 'b : 'a , 'c : 'b , { type Context = IsleContext < 'a , 'b , 'c > ; type Output = (Type , InstructionData) ; fn next (& mut self , ctx : & mut IsleContext < 'a , 'b , 'c >) -> Option < Self :: Output > { while let Some (value) = self . stack . pop () { debug_assert ! (ctx . ctx . func . dfg . value_is_real (value)) ; trace ! ("iter: value {:?}" , value) ; match ctx . ctx . func . dfg . value_def (value) { ValueDef :: Union (x , y) => { debug_assert_ne ! (x , Value :: reserved_value ()) ; debug_assert_ne ! (y , Value :: reserved_value ()) ; trace ! (" -> {}, {}" , x , y) ; self . stack . push (x) ; self . stack . push (y) ; continue ; } ValueDef :: Result (inst , _) if ctx . ctx . func . dfg . inst_results (inst) . len () == 1 => { let ty = ctx . ctx . func . dfg . value_type (value) ; trace ! (" -> value of type {}" , ty) ; return Some ((ty , ctx . ctx . func . dfg . insts [inst])) ; } _ => { } } } None } }
};
}
