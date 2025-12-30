// Generated macro for impl_63 (impl)
macro_rules! Depcrate_fixedimpl_63 {
() => {
// Module: crate::fixed
// Provides: {"impl_63"}
// Dependencies: {}
impl < M > DataProvider < M > for FixedProvider < M > where M : DataMarker , for < 'a > < M :: DataStruct as Yokeable < 'a > > :: Output : Clone , { fn load (& self , _ : DataRequest) -> Result < DataResponse < M > , DataError > { Ok (DataResponse { metadata : Default :: default () , payload : self . data . clone () , }) } }
};
}
