// Generated macro for create_wrapper_function (function)
macro_rules! Depcrate_commoncreate_wrapper_function {
() => {
// Module: crate::common
// Provides: {"create_wrapper_function"}
// Dependencies: {}
pub (crate) fn create_wrapper_function (module : & mut dyn Module , sig : Signature , wrapper_name : & str , callee_name : & str ,) { let wrapper_func_id = module . declare_function (wrapper_name , Linkage :: Export , & sig) . unwrap () ; let callee_func_id = module . declare_function (callee_name , Linkage :: Import , & sig) . unwrap () ; let mut ctx = Context :: new () ; ctx . func . signature = sig ; { let mut func_ctx = FunctionBuilderContext :: new () ; let mut bcx = FunctionBuilder :: new (& mut ctx . func , & mut func_ctx) ; let block = bcx . create_block () ; bcx . switch_to_block (block) ; let func = & mut bcx . func . stencil ; let args = func . signature . params . iter () . map (| param | func . dfg . append_block_param (block , param . value_type)) . collect :: < Vec < Value > > () ; let callee_func_ref = module . declare_func_in_func (callee_func_id , & mut bcx . func) ; let call_inst = bcx . ins () . call (callee_func_ref , & args) ; let results = bcx . inst_results (call_inst) . to_vec () ; bcx . ins () . return_ (& results) ; bcx . seal_all_blocks () ; bcx . finalize () ; } module . define_function (wrapper_func_id , & mut ctx) . unwrap () ; }
};
}
