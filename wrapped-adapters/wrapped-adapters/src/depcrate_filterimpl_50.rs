// Generated macro for impl_50 (impl)
macro_rules! Depcrate_filterimpl_50 {
() => {
// Module: crate::filter
// Provides: {"impl_50"}
// Dependencies: {}
impl < D , F , M > DynamicDataProvider < M > for FilterDataProvider < D , F > where F : Fn (DataIdentifierBorrowed) -> bool , M : DynamicDataMarker , D : DynamicDataProvider < M > , { fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { self . check (marker , req) ? ; self . inner . load_data (marker , req) } }
};
}
