// Generated macro for impl_18 (impl)
macro_rules! Depcrate_core_package_id_specimpl_18 {
() => {
// Module: crate::core::package_id_spec
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for PackageIdSpec { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut printed_name = false ; match self . url { Some (ref url) => { if let Some (protocol) = self . kind . as_ref () . and_then (| k | k . protocol ()) { write ! (f , "{protocol}+") ? ; } write ! (f , "{}" , url) ? ; if let Some (SourceKind :: Git (git_ref)) = self . kind . as_ref () { if let Some (pretty) = git_ref . pretty_ref (true) { write ! (f , "?{}" , pretty) ? ; } } if url . path_segments () . unwrap () . next_back () . unwrap () != & * self . name { printed_name = true ; write ! (f , "#{}" , self . name) ? ; } } None => { printed_name = true ; write ! (f , "{}" , self . name) ? ; } } if let Some (ref v) = self . version { write ! (f , "{}{}" , if printed_name { "@" } else { "#" } , v) ? ; } Ok (()) } }
};
}
