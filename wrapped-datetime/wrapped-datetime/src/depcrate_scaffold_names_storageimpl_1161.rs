// Generated macro for impl_1161 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1161 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1161"}
// Dependencies: {}
impl < M : DynamicDataMarker , Variables > MaybePayload < M , Variables > for () { # [inline] fn new_empty () -> Self { } # [inline] fn load_put < P > (& mut self , _ : & P , _ : DataRequest , _ : Variables ,) -> Result < Result < DataResponseMetadata , DataError > , MaybePayloadError > where P : BoundDataProvider < M > + ? Sized , Self : Sized , { Err (MaybePayloadError :: FormatterTooSpecific) } # [inline] fn get (& self) -> DataPayloadWithVariablesBorrowed < '_ , M , Variables > { DataPayloadWithVariablesBorrowed { inner : OptionalNames :: None , } } }
};
}
