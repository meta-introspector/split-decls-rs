// Generated macro for impl_15 (impl)
macro_rules! Depcrate_schemeimpl_15 {
() => {
// Module: crate::scheme
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a > From < & 'a str > for Scheme { fn from (value : & 'a str) -> Self { match value { "ssh" | "ssh+git" | "git+ssh" => Scheme :: Ssh , "file" => Scheme :: File , "git" => Scheme :: Git , "http" => Scheme :: Http , "https" => Scheme :: Https , unknown => Scheme :: Ext (unknown . into ()) , } } }
};
}
