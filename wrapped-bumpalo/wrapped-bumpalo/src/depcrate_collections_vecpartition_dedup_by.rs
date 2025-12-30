// Generated macro for partition_dedup_by (function)
macro_rules! Depcrate_collections_vecpartition_dedup_by {
() => {
// Module: crate::collections::vec
// Provides: {"partition_dedup_by"}
// Dependencies: {}
fn partition_dedup_by < T , F > (s : & mut [T] , mut same_bucket : F) -> (& mut [T] , & mut [T]) where F : FnMut (& mut T , & mut T) -> bool , { let len = s . len () ; if len <= 1 { return (s , & mut []) ; } let ptr = s . as_mut_ptr () ; let mut next_read : usize = 1 ; let mut next_write : usize = 1 ; unsafe { while next_read < len { let ptr_read = ptr . add (next_read) ; let prev_ptr_write = ptr . add (next_write - 1) ; if ! same_bucket (& mut * ptr_read , & mut * prev_ptr_write) { if next_read != next_write { let ptr_write = prev_ptr_write . offset (1) ; mem :: swap (& mut * ptr_read , & mut * ptr_write) ; } next_write += 1 ; } next_read += 1 ; } } s . split_at_mut (next_write) }
};
}
