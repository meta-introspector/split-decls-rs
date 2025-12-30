// Generated macro for padding_needed_for (function)
macro_rules! Depcrate_arcpadding_needed_for {
() => {
// Module: crate::arc
// Provides: {"padding_needed_for"}
// Dependencies: {}
# [inline] # [must_use] fn padding_needed_for (layout : Layout , align : usize) -> usize { let len = layout . size () ; let len_rounded_up = len . wrapping_add (align) . wrapping_sub (1) & ! align . wrapping_sub (1) ; len_rounded_up . wrapping_sub (len) }
};
}
