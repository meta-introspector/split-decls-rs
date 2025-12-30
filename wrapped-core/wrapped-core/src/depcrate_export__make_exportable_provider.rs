// Generated macro for __make_exportable_provider (macro)
macro_rules! Depcrate_export__make_exportable_provider {
() => {
// Module: crate::export
// Provides: {"__make_exportable_provider"}
// Dependencies: {}
# [doc = " This macro can be used on a data provider to allow it to be exported by `ExportDriver`."] # [doc = ""] # [doc = " Data generation 'compiles' data by using this data provider (which usually translates data from"] # [doc = " different sources and doesn't have to be efficient) to generate data structs, and then writing"] # [doc = " them to an efficient format like `BlobDataProvider` or `BakedDataProvider`. The requirements"] # [doc = " for `make_exportable_provider` are:"] # [doc = " * The data struct has to implement [`serde::Serialize`](::serde::Serialize) and [`databake::Bake`]"] # [doc = " * The provider needs to implement [`IterableDataProvider`] for all specified [`DataMarker`]s."] # [doc = "   This allows the generating code to know which [`DataIdentifierCow`]s to export."] # [macro_export] # [doc (hidden)] macro_rules ! __make_exportable_provider { ($ provider : ty , [$ ($ (# [$ cfg : meta]) ? $ struct_m : ty) ,+,]) => { impl $ crate :: export :: ExportableProvider for $ provider { fn supported_markers (& self) -> alloc :: collections :: BTreeSet <$ crate :: DataMarkerInfo > { alloc :: collections :: BTreeSet :: from_iter ([$ ($ (# [$ cfg]) ? <$ struct_m >:: INFO ,) +]) } } $ crate :: dynutil :: impl_dynamic_data_provider ! ($ provider , [$ ($ (# [$ cfg]) ? $ struct_m) ,+,] , $ crate :: export :: ExportMarker) ; $ crate :: dynutil :: impl_iterable_dynamic_data_provider ! ($ provider , [$ ($ (# [$ cfg]) ? $ struct_m) ,+,] , $ crate :: export :: ExportMarker) ; } ; }
};
}
