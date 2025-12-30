// Generated macro for Error (enum)
macro_rules! Depcrate_flatError {
() => {
// Module: crate::flat
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Denotes invalid flat sample buffers when trying to convert to stricter types."] # [doc = ""] # [doc = " The biggest use case being `ImageBuffer` which expects closely packed"] # [doc = " samples in a row major matrix representation. But this error type may be"] # [doc = " reused for other import functions. A more versatile user may also try to"] # [doc = " correct the underlying representation depending on the error variant."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum Error { # [doc = " The represented image was too large."] # [doc = ""] # [doc = " The optional value denotes a possibly accepted maximal bound."] TooLarge , # [doc = " The represented image can not use this representation."] # [doc = ""] # [doc = " Has an additional value of the normalized form that would be accepted."] NormalFormRequired (NormalForm) , # [doc = " The color format did not match the channel count."] # [doc = ""] # [doc = " In some cases you might be able to fix this by lowering the reported pixel count of the"] # [doc = " buffer without touching the strides."] # [doc = ""] # [doc = " In very special circumstances you *may* do the opposite. This is **VERY** dangerous but not"] # [doc = " directly memory unsafe although that will likely alias pixels. One scenario is when you"] # [doc = " want to construct an `Rgba` image but have only 3 bytes per pixel and for some reason don't"] # [doc = " care about the value of the alpha channel even though you need `Rgba`."] ChannelCountMismatch (u8 , u8) , # [doc = " Deprecated - `ChannelCountMismatch` is used instead"] WrongColor (ColorType) , }
};
}
