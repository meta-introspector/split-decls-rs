// Generated macro for impl_210 (impl)
macro_rules! Depcrate_machinst_abiimpl_210 {
() => {
// Module: crate::machinst::abi
// Provides: {"impl_210"}
// Dependencies: {}
impl FrameLayout { # [doc = " Split the clobbered callee-save registers into integer-class and"] # [doc = " float-class groups."] # [doc = ""] # [doc = " This method does not currently support vector-class callee-save"] # [doc = " registers because no current backend has them."] pub fn clobbered_callee_saves_by_class (& self) -> (& [Writable < RealReg >] , & [Writable < RealReg >]) { let (ints , floats) = self . clobbered_callee_saves . split_at (self . clobbered_callee_saves . partition_point (| r | r . to_reg () . class () == RegClass :: Int) ,) ; debug_assert ! (floats . iter () . all (| r | r . to_reg () . class () == RegClass :: Float)) ; (ints , floats) } # [doc = " The size of FP to SP while the frame is active (not during prologue"] # [doc = " setup or epilogue tear down)."] pub fn active_size (& self) -> u32 { self . outgoing_args_size + self . fixed_frame_storage_size + self . clobber_size } # [doc = " Get the offset from the SP to the sized stack slots area."] pub fn sp_to_sized_stack_slots (& self) -> u32 { self . outgoing_args_size } }
};
}
