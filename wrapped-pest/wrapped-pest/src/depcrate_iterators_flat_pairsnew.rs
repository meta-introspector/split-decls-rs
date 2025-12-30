// Generated macro for new (function)
macro_rules! Depcrate_iterators_flat_pairsnew {
() => {
// Module: crate::iterators::flat_pairs
// Provides: {"new"}
// Dependencies: {}
pub fn new < 'i , R : RuleType > (queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , line_index : Rc < LineIndex > , start : usize , end : usize ,) -> FlatPairs < 'i , R > { FlatPairs { queue , input , line_index , start , end , } }
};
}
