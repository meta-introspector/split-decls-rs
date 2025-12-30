// Generated macro for on_char_typed_ (function)
macro_rules! Depcrate_typingon_char_typed_ {
() => {
// Module: crate::typing
// Provides: {"on_char_typed_"}
// Dependencies: {}
fn on_char_typed_ (file : & Parse < SourceFile > , offset : TextSize , char_typed : char , edition : Edition ,) -> Option < ExtendedTextEdit > { match char_typed { '.' => on_dot_typed (& file . tree () , offset) , '=' => on_eq_typed (& file . tree () , offset) , '>' => on_right_angle_typed (& file . tree () , offset) , '{' | '(' | '<' => on_opening_delimiter_typed (file , offset , char_typed , edition) , '|' => on_pipe_typed (& file . tree () , offset) , '+' => on_plus_typed (& file . tree () , offset) , _ => None , } . map (conv) }
};
}
