// Generated macro for LineTruncator (struct)
macro_rules! Depcrate_reflowLineTruncator {
() => {
// Module: crate::reflow
// Provides: {"LineTruncator"}
// Dependencies: {}
# [doc = " A state machine that truncates overhanging lines."] # [derive (Debug , Default , Clone)] pub struct LineTruncator < 'a , O , I > where O : Iterator < Item = (I , Alignment) > , I : Iterator < Item = StyledGrapheme < 'a > > , { # [doc = " The given, unprocessed lines"] input_lines : O , max_line_width : u16 , current_line : Vec < StyledGrapheme < 'a > > , # [doc = " Record the offset to skip render"] horizontal_offset : u16 , }
};
}
