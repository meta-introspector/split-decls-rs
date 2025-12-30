// Generated macro for impl_53 (impl)
macro_rules! Depcrate_filterimpl_53 {
() => {
// Module: crate::filter
// Provides: {"impl_53"}
// Dependencies: {}
impl < D , F , M > DryDataProvider < M > for FilterDataProvider < D , F > where F : Fn (DataIdentifierBorrowed) -> bool , M : DataMarker , D : DryDataProvider < M > , { fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { self . check (M :: INFO , req) ? ; self . inner . dry_load (req) } }
};
}
