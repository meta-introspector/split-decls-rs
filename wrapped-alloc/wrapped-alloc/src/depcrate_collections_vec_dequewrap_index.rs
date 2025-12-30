// Generated macro for wrap_index (function)
macro_rules! Depcrate_collections_vec_dequewrap_index {
() => {
// Module: crate::collections::vec_deque
// Provides: {"wrap_index"}
// Dependencies: {}
# [doc = " Returns the index in the underlying buffer for a given logical element index."] # [inline] fn wrap_index (logical_index : usize , capacity : usize) -> usize { debug_assert ! ((logical_index == 0 && capacity == 0) || logical_index < capacity || (logical_index - capacity) < capacity) ; if logical_index >= capacity { logical_index - capacity } else { logical_index } }
};
}
