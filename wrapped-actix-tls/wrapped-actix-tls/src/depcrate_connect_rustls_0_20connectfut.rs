// Generated macro for ConnectFut (enum)
macro_rules! Depcrate_connect_rustls_0_20ConnectFut {
() => {
// Module: crate::connect::rustls_0_20
// Provides: {"ConnectFut"}
// Dependencies: {}
# [doc = " Connect future for Rustls service."] # [doc (hidden)] # [allow (clippy :: large_enum_variant)] pub enum ConnectFut < R , IO > { # [doc = " See issue <https://github.com/briansmith/webpki/issues/54>"] InvalidDns , Future { connect : RustlsConnect < IO > , connection : Option < Connection < R , () > > , } , }
};
}
