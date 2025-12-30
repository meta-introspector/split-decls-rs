// Generated macro for impl_98 (impl)
macro_rules! Depcrate_typesimpl_98 {
() => {
// Module: crate::types
// Provides: {"impl_98"}
// Dependencies: {}
impl SourceItemOrderingModuleItemGroupings { fn build_lut (groups : & [(String , Vec < SourceItemOrderingModuleItemKind >)] ,) -> HashMap < SourceItemOrderingModuleItemKind , usize > { let mut lut = HashMap :: new () ; for (group_index , (_ , items)) in groups . iter () . enumerate () { for item in items { lut . insert (item . clone () , group_index) ; } } lut } fn build_back_lut (groups : & [(String , Vec < SourceItemOrderingModuleItemKind >)] ,) -> HashMap < SourceItemOrderingModuleItemKind , String > { let mut lut = HashMap :: new () ; for (group_name , items) in groups { for item in items { lut . insert (item . clone () , group_name . clone ()) ; } } lut } pub fn grouping_name_of (& self , item : & SourceItemOrderingModuleItemKind) -> Option < & String > { self . back_lut . get (item) } pub fn grouping_names (& self) -> Vec < String > { self . groups . iter () . map (| (name , _) | name . clone ()) . collect () } pub fn is_grouping (& self , grouping : & str) -> bool { self . groups . iter () . any (| (g , _) | g == grouping) } pub fn module_level_order_of (& self , item : & SourceItemOrderingModuleItemKind) -> Option < usize > { self . lut . get (item) . copied () } }
};
}
