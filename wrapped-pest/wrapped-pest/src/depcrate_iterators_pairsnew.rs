// Generated macro for new (function)
macro_rules! Depcrate_iterators_pairsnew {
() => {
// Module: crate::iterators::pairs
// Provides: {"new"}
// Dependencies: {}
pub fn new < 'i , R : RuleType > (queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , line_index : Option < Rc < LineIndex > > , start : usize , end : usize ,) -> Pairs < 'i , R > { let line_index = match line_index { Some (line_index) => line_index , None => { let last_input_pos = queue . last () . map (| token | match * token { QueueableToken :: Start { input_pos , .. } | QueueableToken :: End { input_pos , .. } => input_pos , }) . unwrap_or (0) ; Rc :: new (LineIndex :: new (& input [.. last_input_pos])) } } ; let mut pairs_count = 0 ; let mut cursor = start ; while cursor < end { cursor = match queue [cursor] { QueueableToken :: Start { end_token_index , .. } => end_token_index , _ => unreachable ! () , } + 1 ; pairs_count += 1 ; } Pairs { queue , input , start , end , pairs_count , line_index , } }
};
}
