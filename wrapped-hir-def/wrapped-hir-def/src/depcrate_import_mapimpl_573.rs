// Generated macro for impl_573 (impl)
macro_rules! Depcrate_import_mapimpl_573 {
() => {
// Module: crate::import_map
// Provides: {"impl_573"}
// Dependencies: {}
impl fmt :: Debug for ImportMap { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut importable_names : Vec < _ > = self . item_to_info_map . iter () . map (| (item , (infos , _)) | { let l = infos . len () ; match item { ItemInNs :: Types (it) => format ! ("- {it:?} (t) [{l}]" ,) , ItemInNs :: Values (it) => format ! ("- {it:?} (v) [{l}]" ,) , ItemInNs :: Macros (it) => format ! ("- {it:?} (m) [{l}]" ,) , } }) . collect () ; importable_names . sort () ; f . write_str (& importable_names . join ("\n")) } }
};
}
