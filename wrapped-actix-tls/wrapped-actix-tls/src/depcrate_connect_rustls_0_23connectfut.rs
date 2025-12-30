// Generated macro for ConnectFut (enum)
macro_rules! Depcrate_connect_rustls_0_23ConnectFut {
() => {
// Module: crate::connect::rustls_0_23
// Provides: {"ConnectFut"}
// Dependencies: {}
# [doc = " Connect future for Rustls service."] # [doc (hidden)] # [allow (clippy :: large_enum_variant)] pub enum ConnectFut < R , IO > { InvalidServerName , Future { connect : RustlsConnect < IO > , connection : Option < Connection < R , () > > , } , }
};
}
