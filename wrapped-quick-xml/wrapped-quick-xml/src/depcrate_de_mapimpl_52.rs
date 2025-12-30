// Generated macro for impl_52 (impl)
macro_rules! Depcrate_de_mapimpl_52 {
() => {
// Module: crate::de::map
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'de > TagFilter < 'de > { fn is_suitable (& self , start : & BytesStart) -> Result < bool , DeError > { match self { Self :: Include (n) => Ok (n . name () == start . name ()) , Self :: Exclude (fields , _) => not_in (fields , start) , } } const fn need_skip_text (& self) -> bool { match self { Self :: Include (_) => true , Self :: Exclude (_ , has_text_field) => * has_text_field , } } }
};
}
