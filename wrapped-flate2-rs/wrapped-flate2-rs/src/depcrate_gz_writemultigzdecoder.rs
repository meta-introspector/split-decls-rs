// Generated macro for MultiGzDecoder (struct)
macro_rules! Depcrate_gz_writeMultiGzDecoder {
() => {
// Module: crate::gz::write
// Provides: {"MultiGzDecoder"}
// Dependencies: {}
# [doc = " A gzip streaming decoder that decodes a [gzip file] with multiple members."] # [doc = ""] # [doc = " This structure exposes a [`Write`] interface that will consume compressed data and"] # [doc = " write uncompressed data to the underlying writer."] # [doc = ""] # [doc = " A gzip file consists of a series of *members* concatenated one after another."] # [doc = " `MultiGzDecoder` decodes all members of a file and writes them to the"] # [doc = " underlying writer one after another."] # [doc = ""] # [doc = " To handle members separately, see [GzDecoder] or read more"] # [doc = " [in the introduction](../index.html#about-multi-member-gzip-files)."] # [doc = ""] # [doc = " [gzip file]: https://www.rfc-editor.org/rfc/rfc1952#page-5"] # [derive (Debug)] pub struct MultiGzDecoder < W : Write > { inner : GzDecoder < W > , }
};
}
