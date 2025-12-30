// Generated macro for Template (struct)
macro_rules! Depcrate_templateTemplate {
() => {
// Module: crate::template
// Provides: {"Template"}
// Dependencies: {}
# [doc = " Structure representing a parsed template. It holds the bytecode program for rendering the"] # [doc = " template as well as the length of the original template string, which is used as a guess to"] # [doc = " pre-size the output string buffer."] pub (crate) struct Template < 'template > { original_text : & 'template str , instructions : Vec < Instruction < 'template > > , template_len : usize , }
};
}
