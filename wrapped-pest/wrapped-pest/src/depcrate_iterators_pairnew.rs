// Generated macro for new (function)
macro_rules! Depcrate_iterators_pairnew {
() => {
// Module: crate::iterators::pair
// Provides: {"new"}
// Dependencies: {}
pub fn new < 'i , R : RuleType > (queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , line_index : Rc < LineIndex > , start : usize ,) -> Pair < 'i , R > { Pair { queue , input , start , line_index , } }
};
}
