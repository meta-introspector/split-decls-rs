// Generated macro for BufWriterWithLineEndingFix (struct)
macro_rules! DepcrateBufWriterWithLineEndingFix {
() => {
// Module: crate
// Provides: {"BufWriterWithLineEndingFix"}
// Dependencies: {}
# [doc = " A small helper class to convert LF to CRLF on Windows."] # [doc = " Workaround for <https://github.com/serde-rs/json/issues/535>"] pub struct BufWriterWithLineEndingFix < W : Write > { inner : BufWriter < W > , # [cfg (windows)] last_written : Option < u8 > , }
};
}
