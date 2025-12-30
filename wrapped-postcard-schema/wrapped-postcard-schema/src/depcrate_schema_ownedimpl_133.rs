// Generated macro for impl_133 (impl)
macro_rules! Depcrate_schema_ownedimpl_133 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_133"}
// Dependencies: {}
impl From < & DataModelVariant > for OwnedDataModelVariant { fn from (value : & DataModelVariant) -> Self { match value { DataModelVariant :: UnitVariant => Self :: UnitVariant , DataModelVariant :: NewtypeVariant (d) => Self :: NewtypeVariant (Box :: new ((* d) . into ())) , DataModelVariant :: TupleVariant (d) => { Self :: TupleVariant (d . iter () . map (| i | (* i) . into ()) . collect ()) } DataModelVariant :: StructVariant (d) => { Self :: StructVariant (d . iter () . map (| i | (* i) . into ()) . collect ()) } } } }
};
}
