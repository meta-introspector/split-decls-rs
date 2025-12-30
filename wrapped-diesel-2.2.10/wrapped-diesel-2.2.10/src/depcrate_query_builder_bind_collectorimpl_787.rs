// Generated macro for impl_787 (impl)
macro_rules! Depcrate_query_builder_bind_collectorimpl_787 {
() => {
// Module: crate::query_builder::bind_collector
// Provides: {"impl_787"}
// Dependencies: {}
impl < DB > MoveableBindCollector < DB > for RawBytesBindCollector < DB > where for < 'a > DB : Backend < BindCollector < 'a > = Self > + TypeMetadata + 'static , < DB as TypeMetadata > :: TypeMetadata : Clone + Send , { type BindData = Self ; fn moveable (& self) -> Self :: BindData { RawBytesBindCollector { binds : self . binds . clone () , metadata : self . metadata . clone () , } } fn append_bind_data (& mut self , from : & Self :: BindData) { self . binds . extend (from . binds . iter () . cloned ()) ; self . metadata . extend (from . metadata . clone ()) ; } }
};
}
