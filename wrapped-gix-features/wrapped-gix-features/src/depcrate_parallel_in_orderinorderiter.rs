// Generated macro for InOrderIter (struct)
macro_rules! Depcrate_parallel_in_orderInOrderIter {
() => {
// Module: crate::parallel::in_order
// Provides: {"InOrderIter"}
// Dependencies: {}
# [doc = " An iterator which olds iterated items with a **sequential** ID starting at 0 long enough to dispense them in order."] pub struct InOrderIter < T , I > { # [doc = " The iterator yielding the out-of-order elements we are to yield in order."] pub inner : I , store : BTreeMap < SequenceId , T > , next_chunk : SequenceId , is_done : bool , }
};
}
