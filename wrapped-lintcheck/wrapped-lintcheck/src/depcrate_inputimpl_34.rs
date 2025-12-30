// Generated macro for impl_34 (impl)
macro_rules! Depcrate_inputimpl_34 {
() => {
// Module: crate::input
// Provides: {"impl_34"}
// Dependencies: {}
impl TomlCrate { fn file_link (& self , default : & str) -> String { let mut link = self . online_link . clone () . unwrap_or_else (| | default . to_string ()) ; link = link . replace ("{krate}" , & self . name) ; link = link . replace ("{krate_}" , & self . name . replace ('-' , "_")) ; if let Some (version) = & self . version { link = link . replace ("{version}" , version) ; } if let Some (url) = & self . git_url { link = link . replace ("{url}" , url) ; } if let Some (hash) = & self . git_hash { link = link . replace ("{hash}" , hash) ; } if let Some (path) = & self . path { link = link . replace ("{path}" , path) ; } link } }
};
}
