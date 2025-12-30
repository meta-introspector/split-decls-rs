// Generated macro for ElementWriter (struct)
macro_rules! Depcrate_writerElementWriter {
() => {
// Module: crate::writer
// Provides: {"ElementWriter"}
// Dependencies: {}
# [doc = " A struct to write an element. Contains methods to add attributes and inner"] # [doc = " elements to the element"] pub struct ElementWriter < 'a , W > { writer : & 'a mut Writer < W > , start_tag : BytesStart < 'a > , state : AttributeIndent , # [doc = " Contains spaces used to write space indents of attributes"] spaces : Vec < u8 > , }
};
}
