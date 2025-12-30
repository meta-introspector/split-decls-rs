// Generated macro for impl_89 (impl)
macro_rules! Depcrate_discoverimpl_89 {
() => {
// Module: crate::discover
// Provides: {"impl_89"}
// Dependencies: {}
impl CargoParser < DiscoverProjectMessage > for DiscoverProjectParser { fn from_line (& self , line : & str , _error : & mut String) -> Option < DiscoverProjectMessage > { match serde_json :: from_str :: < DiscoverProjectData > (line) { Ok (data) => { let msg = DiscoverProjectMessage :: new (data) ; Some (msg) } Err (err) => { let err = DiscoverProjectData :: Error { error : format ! ("{err:#?}\n{line}") , source : None } ; Some (DiscoverProjectMessage :: new (err)) } } } fn from_eof (& self) -> Option < DiscoverProjectMessage > { None } }
};
}
