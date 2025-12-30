// Generated macro for nghttp2_extension (struct)
macro_rules! Depcratenghttp2_extension {
() => {
// Module: crate
// Provides: {"nghttp2_extension"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The extension frame.  It has following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_extension { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The pointer to extension payload.  The exact pointer type is"] # [doc = " determined by hd.type."] # [doc = ""] # [doc = " Currently, no extension is supported.  This is a place holder for"] # [doc = " the future extensions."] pub payload : * mut :: std :: os :: raw :: c_void , }
};
}
