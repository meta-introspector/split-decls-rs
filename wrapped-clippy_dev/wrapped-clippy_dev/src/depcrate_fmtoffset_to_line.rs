// Generated macro for offset_to_line (function)
macro_rules! Depcrate_fmtoffset_to_line {
() => {
// Module: crate::fmt
// Provides: {"offset_to_line"}
// Dependencies: {}
fn offset_to_line (text : & str , offset : usize) -> usize { match text . split ('\n') . try_fold ((1usize , 0usize) , | (line , pos) , s | { let pos = pos + s . len () + 1 ; if pos > offset { ControlFlow :: Break (line) } else { ControlFlow :: Continue ((line + 1 , pos)) } }) { ControlFlow :: Break (x) | ControlFlow :: Continue ((x , _)) => x , } }
};
}
