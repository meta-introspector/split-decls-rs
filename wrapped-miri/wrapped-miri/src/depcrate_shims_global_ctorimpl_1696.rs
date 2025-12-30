// Generated macro for impl_1696 (impl)
macro_rules! Depcrate_shims_global_ctorimpl_1696 {
() => {
// Module: crate::shims::global_ctor
// Provides: {"impl_1696"}
// Dependencies: {}
impl < 'tcx > GlobalCtorState < 'tcx > { pub fn on_stack_empty (& mut self , this : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , Poll < () > > { use GlobalCtorStatePriv :: * ; let new_state = 'new_state : { match & mut self . 0 { Init => { let this = this . eval_context_mut () ; let ctors = match this . tcx . sess . target . binary_format { BinaryFormat :: Coff => this . lookup_link_section (| section | section == ".CRT$XCU") ? , BinaryFormat :: MachO => this . lookup_link_section (| section | { let mut parts = section . splitn (3 , ',') ; let (segment_name , section_name , section_type) = (parts . next () , parts . next () , parts . next ()) ; segment_name == Some ("__DATA") && section_name == Some ("__mod_init_func") && matches ! (section_type , None | Some ("mod_init_funcs")) }) ? , BinaryFormat :: Elf | BinaryFormat :: Wasm => this . lookup_link_section (| section | section == ".init_array") ? , _ => break 'new_state Done , } ; break 'new_state Ctors (ctors) ; } Ctors (ctors) => { if let Some (ctor) = ctors . pop () { let this = this . eval_context_mut () ; let ctor = ctor . to_scalar () . to_pointer (this) ? ; let thread_callback = this . get_ptr_fn (ctor) ? . as_instance () ? ; this . call_function (thread_callback , ExternAbi :: C { unwind : false } , & [] , None , ReturnContinuation :: Stop { cleanup : true } ,) ? ; return interp_ok (Poll :: Pending) ; } break 'new_state Done ; } Done => return interp_ok (Poll :: Ready (())) , } } ; self . 0 = new_state ; interp_ok (Poll :: Pending) } }
};
}
