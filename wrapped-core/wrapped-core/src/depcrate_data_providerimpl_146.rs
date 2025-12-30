// Generated macro for impl_146 (impl)
macro_rules! Depcrate_data_providerimpl_146 {
() => {
// Module: crate::data_provider
// Provides: {"impl_146"}
// Dependencies: {}
impl < M , M0 , Y , P > BoundDataProvider < M0 > for DataProviderWithMarker < M , P > where M : DataMarker < DataStruct = Y > , M0 : DynamicDataMarker < DataStruct = Y > , Y : for < 'a > Yokeable < 'a > , P : DataProvider < M > , { # [inline] fn load_bound (& self , req : DataRequest) -> Result < DataResponse < M0 > , DataError > { self . inner . load (req) . map (DataResponse :: cast) } # [inline] fn bound_marker (& self) -> DataMarkerInfo { M :: INFO } }
};
}
