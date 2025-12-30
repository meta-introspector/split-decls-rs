// Generated macro for impl_182 (impl)
macro_rules! Depcrate_setimpl_182 {
() => {
// Module: crate::set
// Provides: {"impl_182"}
// Dependencies: {}
# [cfg (feature = "typesize")] impl < K , S > typesize :: TypeSize for DashSet < K , S > where K : typesize :: TypeSize + Eq + Hash , S : typesize :: TypeSize + Clone + BuildHasher , { fn extra_size (& self) -> usize { self . inner . extra_size () } typesize :: if_typesize_details ! { fn get_collection_item_count (& self) -> Option < usize > { Some (self . len ()) } } }
};
}
