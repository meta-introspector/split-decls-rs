// Generated macro for steal_from_freelist (function)
macro_rules! Depcrate_object_implssteal_from_freelist {
() => {
// Module: crate::object::impls
// Provides: {"steal_from_freelist"}
// Dependencies: {}
# [doc = " In conjunction with the handles free list, leaving an empty Vec in place of the original causes it to not be"] # [doc = " returned to the free list."] fn steal_from_freelist (data : & mut Vec < u8 >) -> Vec < u8 > { std :: mem :: take (data) }
};
}
