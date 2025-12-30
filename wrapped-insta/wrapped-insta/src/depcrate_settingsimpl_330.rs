// Generated macro for impl_330 (impl)
macro_rules! Depcrate_settingsimpl_330 {
() => {
// Module: crate::settings
// Provides: {"impl_330"}
// Dependencies: {}
impl ActualSettings { pub fn sort_maps (& mut self , value : bool) { self . sort_maps = value ; } pub fn snapshot_path < P : AsRef < Path > > (& mut self , path : P) { self . snapshot_path = path . as_ref () . to_path_buf () ; } pub fn snapshot_suffix < I : Into < String > > (& mut self , suffix : I) { self . snapshot_suffix = suffix . into () ; } pub fn input_file < P : AsRef < Path > > (& mut self , p : P) { self . input_file = Some (p . as_ref () . to_path_buf ()) ; } pub fn description < S : Into < String > > (& mut self , value : S) { self . description = Some (value . into ()) ; } # [cfg (feature = "serde")] pub fn info < S : Serialize > (& mut self , s : & S) { let serializer = ContentSerializer :: < ValueError > :: new () ; let content = Serialize :: serialize (s , serializer) . unwrap () ; # [cfg (feature = "redactions")] let content = self . redactions . apply_to_content (content) ; self . info = Some (content) ; } pub fn raw_info (& mut self , content : & Content) { self . info = Some (content . to_owned ()) ; } pub fn omit_expression (& mut self , value : bool) { self . omit_expression = value ; } pub fn prepend_module_to_snapshot (& mut self , value : bool) { self . prepend_module_to_snapshot = value ; } # [cfg (feature = "redactions")] pub fn redactions < R : Into < Redactions > > (& mut self , r : R) { self . redactions = r . into () ; } # [cfg (feature = "filters")] pub fn filters < F : Into < Filters > > (& mut self , f : F) { self . filters = f . into () ; } # [cfg (feature = "glob")] pub fn allow_empty_glob (& mut self , value : bool) { self . allow_empty_glob = value ; } }
};
}
