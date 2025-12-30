// Generated macro for impl_51 (impl)
macro_rules! Depcrate_filterimpl_51 {
() => {
// Module: crate::filter
// Provides: {"impl_51"}
// Dependencies: {}
impl < D , F , M > DynamicDryDataProvider < M > for FilterDataProvider < D , F > where F : Fn (DataIdentifierBorrowed) -> bool , M : DynamicDataMarker , D : DynamicDryDataProvider < M > , { fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { self . check (marker , req) ? ; self . inner . dry_load_data (marker , req) } }
};
}
