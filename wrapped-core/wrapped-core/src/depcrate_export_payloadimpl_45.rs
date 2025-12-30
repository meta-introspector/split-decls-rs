// Generated macro for impl_45 (impl)
macro_rules! Depcrate_export_payloadimpl_45 {
() => {
// Module: crate::export::payload
// Provides: {"impl_45"}
// Dependencies: {}
impl < M > UpcastDataPayload < M > for ExportMarker where M : DynamicDataMarker , M :: DataStruct : Sync + Send , for < 'a > < M :: DataStruct as Yokeable < 'a > > :: Output : Bake + BakeSize + serde :: Serialize + MaybeEncodeAsVarULE + PartialEq , { fn upcast (other : DataPayload < M >) -> DataPayload < ExportMarker > { DataPayload :: from_owned (ExportBox { payload : Arc :: new (other) , }) } }
};
}
