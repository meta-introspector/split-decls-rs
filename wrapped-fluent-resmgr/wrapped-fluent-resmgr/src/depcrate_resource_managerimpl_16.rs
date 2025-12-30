// Generated macro for impl_16 (impl)
macro_rules! Depcrate_resource_managerimpl_16 {
() => {
// Module: crate::resource_manager
// Provides: {"impl_16"}
// Dependencies: {}
impl Iterator for BundleIter { type Item = FluentBundleResult < FluentResource > ; fn next (& mut self) -> Option < Self :: Item > { let locale = self . locales . next () ? ; let mut bundle = FluentBundle :: new (vec ! [locale . clone ()]) ; for res_id in self . res_ids . iter () { let full_path = format ! ("./tests/resources/{}/{}" , locale , res_id) ; let source = fs :: read_to_string (full_path) . unwrap () ; let res = FluentResource :: try_new (source) . unwrap () ; bundle . add_resource (res) . unwrap () ; } Some (Ok (bundle)) } }
};
}
