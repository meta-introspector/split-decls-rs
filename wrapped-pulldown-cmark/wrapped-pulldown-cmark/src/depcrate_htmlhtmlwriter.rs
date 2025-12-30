// Generated macro for HtmlWriter (struct)
macro_rules! Depcrate_htmlHtmlWriter {
() => {
// Module: crate::html
// Provides: {"HtmlWriter"}
// Dependencies: {}
struct HtmlWriter < 'a , I , W > { # [doc = " Iterator supplying events."] iter : I , # [doc = " Writer to write to."] writer : W , # [doc = " Whether or not the last write wrote a newline."] end_newline : bool , # [doc = " Whether if inside a metadata block (text should not be written)"] in_non_writing_block : bool , table_state : TableState , table_alignments : Vec < Alignment > , table_cell_index : usize , numbers : HashMap < CowStr < 'a > , usize > , }
};
}
