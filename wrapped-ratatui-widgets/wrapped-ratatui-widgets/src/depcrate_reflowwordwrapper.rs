// Generated macro for WordWrapper (struct)
macro_rules! Depcrate_reflowWordWrapper {
() => {
// Module: crate::reflow
// Provides: {"WordWrapper"}
// Dependencies: {}
# [doc = " A state machine that wraps lines on word boundaries."] # [derive (Debug , Default , Clone)] pub struct WordWrapper < 'a , O , I > where O : Iterator < Item = (I , Alignment) > , I : Iterator < Item = StyledGrapheme < 'a > > , { # [doc = " The given, unprocessed lines"] input_lines : O , max_line_width : u16 , wrapped_lines : VecDeque < Vec < StyledGrapheme < 'a > > > , current_alignment : Alignment , current_line : Vec < StyledGrapheme < 'a > > , # [doc = " Removes the leading whitespace from lines"] trim : bool , pending_word : Vec < StyledGrapheme < 'a > > , pending_whitespace : VecDeque < StyledGrapheme < 'a > > , pending_line_pool : Vec < Vec < StyledGrapheme < 'a > > > , }
};
}
