// Generated macro for call_inline_asm (function)
macro_rules! Depcrate_inline_asmcall_inline_asm {
() => {
// Module: crate::inline_asm
// Provides: {"call_inline_asm"}
// Dependencies: {}
fn call_inline_asm < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , asm_name : & str , slot_size : Size , inputs : Vec < (Size , Value) > , outputs : Vec < (Size , CPlace < 'tcx >) > ,) { let stack_slot = fx . create_stack_slot (u32 :: try_from (slot_size . bytes () . next_multiple_of (16)) . unwrap () , 16) ; let inline_asm_func = fx . module . declare_function (asm_name , Linkage :: Import , & Signature { call_conv : CallConv :: SystemV , params : vec ! [AbiParam :: new (fx . pointer_type)] , returns : vec ! [] , } ,) . unwrap () ; let inline_asm_func = fx . module . declare_func_in_func (inline_asm_func , fx . bcx . func) ; if fx . clif_comments . enabled () { fx . add_comment (inline_asm_func , asm_name) ; } for (offset , value) in inputs { stack_slot . offset (fx , i32 :: try_from (offset . bytes ()) . unwrap () . into ()) . store (fx , value , MemFlags :: trusted () ,) ; } let stack_slot_addr = stack_slot . get_addr (fx) ; fx . bcx . ins () . call (inline_asm_func , & [stack_slot_addr]) ; for (offset , place) in outputs { let ty = if place . layout () . ty . is_simd () { let (lane_count , lane_type) = place . layout () . ty . simd_size_and_type (fx . tcx) ; asm_clif_type (fx , lane_type) . unwrap () . by (lane_count . try_into () . unwrap ()) . unwrap () } else { asm_clif_type (fx , place . layout () . ty) . unwrap () } ; let value = stack_slot . offset (fx , i32 :: try_from (offset . bytes ()) . unwrap () . into ()) . load (fx , ty , MemFlags :: trusted () ,) ; place . write_cvalue (fx , CValue :: by_val (value , place . layout ())) ; } }
};
}
