// Generated macro for align_to (function)
macro_rules! Depcrate_machinst_helpersalign_to {
() => {
// Module: crate::machinst::helpers
// Provides: {"align_to"}
// Dependencies: {}
# [doc = " Align a size up to a power-of-two alignment."] pub (crate) fn align_to < N > (x : N , alignment : N) -> N where N : Not < Output = N > + BitAnd < N , Output = N > + Add < N , Output = N > + Sub < N , Output = N > + From < u8 > + Copy , { let alignment_mask = alignment - 1 . into () ; (x + alignment_mask) & ! alignment_mask }
};
}
