// Generated macro for extend_layout (function)
macro_rules! Depcrate_arcextend_layout {
() => {
// Module: crate::arc
// Provides: {"extend_layout"}
// Dependencies: {}
# [inline] fn extend_layout (layout : Layout , next : Layout) -> Option < (Layout , usize) > { let new_align = cmp :: max (layout . align () , next . align ()) ; let pad = padding_needed_for (layout , next . align ()) ; let offset = layout . size () . checked_add (pad) ? ; let new_size = offset . checked_add (next . size ()) ? ; let layout = Layout :: from_size_align (new_size , new_align) . ok () ? ; Some ((layout , offset)) }
};
}
