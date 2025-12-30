// Generated macro for ConsumingIter (struct)
macro_rules! Depcrate_hash_setConsumingIter {
() => {
// Module: crate::hash::set
// Provides: {"ConsumingIter"}
// Dependencies: {}
# [doc = " A consuming iterator over the elements of a set."] pub struct ConsumingIter < A > where A : Hash + Eq + Clone , { it : NodeDrain < Value < A > > , }
};
}
