// Generated macro for impl_52 (impl)
macro_rules! Depcrate_filterimpl_52 {
() => {
// Module: crate::filter
// Provides: {"impl_52"}
// Dependencies: {}
impl < D , F , M > DataProvider < M > for FilterDataProvider < D , F > where F : Fn (DataIdentifierBorrowed) -> bool , M : DataMarker , D : DataProvider < M > , { fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { self . check (M :: INFO , req) ? ; self . inner . load (req) } }
};
}
