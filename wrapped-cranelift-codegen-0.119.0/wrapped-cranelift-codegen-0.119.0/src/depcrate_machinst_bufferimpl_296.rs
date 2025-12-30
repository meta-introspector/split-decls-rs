// Generated macro for impl_296 (impl)
macro_rules! Depcrate_machinst_bufferimpl_296 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_296"}
// Dependencies: {}
impl < I : VCodeInst > MachTextSectionBuilder < I > { # [doc = " Creates a new text section builder which will have `num_funcs` functions"] # [doc = " pushed into it."] pub fn new (num_funcs : usize) -> MachTextSectionBuilder < I > { let mut buf = MachBuffer :: new () ; buf . reserve_labels_for_blocks (num_funcs) ; MachTextSectionBuilder { buf , next_func : 0 , force_veneers : ForceVeneers :: No , } } }
};
}
