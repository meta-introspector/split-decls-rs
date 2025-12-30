// Generated macro for impl_2557 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2557 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2557"}
// Dependencies: {}
impl Amode { # [doc = " Add the registers referenced by this Amode to `collector`."] pub (crate) fn get_operands (& mut self , collector : & mut impl OperandVisitor) { match self { Amode :: RegOffset { base , offset : _ } => collector . reg_use (base) , Amode :: SpOffset { .. } | Amode :: Stack { .. } => { } } } pub (crate) fn get_base_register (& self) -> Option < XReg > { match self { Amode :: RegOffset { base , offset : _ } => Some ((* base) . into ()) , Amode :: SpOffset { .. } | Amode :: Stack { .. } => Some (XReg :: new (stack_reg ()) . unwrap ()) , } } pub (crate) fn get_offset_with_state < P > (& self , state : & EmitState < P >) -> i32 where P : PulleyTargetKind , { match self { Amode :: RegOffset { base : _ , offset } | Amode :: SpOffset { offset } => * offset , Amode :: Stack { amode } => { let offset64 = match amode { StackAMode :: IncomingArg (offset , stack_args_size) => { let offset = i64 :: from (* stack_args_size) - * offset ; let frame_layout = state . frame_layout () ; let sp_offset = frame_layout . tail_args_size + frame_layout . setup_area_size + frame_layout . clobber_size + frame_layout . fixed_frame_storage_size + frame_layout . outgoing_args_size ; i64 :: from (sp_offset) - offset } StackAMode :: Slot (offset) => { offset + i64 :: from (state . frame_layout () . outgoing_args_size) } StackAMode :: OutgoingArg (offset) => * offset , } ; i32 :: try_from (offset64) . unwrap () } } } }
};
}
