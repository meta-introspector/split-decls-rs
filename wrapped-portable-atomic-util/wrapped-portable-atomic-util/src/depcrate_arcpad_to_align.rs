// Generated macro for pad_to_align (function)
macro_rules! Depcrate_arcpad_to_align {
() => {
// Module: crate::arc
// Provides: {"pad_to_align"}
// Dependencies: {}
# [inline] # [must_use] fn pad_to_align (layout : Layout) -> Layout { let pad = padding_needed_for (layout , layout . align ()) ; let new_size = layout . size () + pad ; unsafe { Layout :: from_size_align_unchecked (new_size , layout . align ()) } }
};
}
