// Generated macro for impl_548 (impl)
macro_rules! Depcrate_expression_select_byimpl_548 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_548"}
// Dependencies: {}
impl < T , DB > QueryMetadata < SelectBy < T , DB > > for DB where DB : Backend , T : Selectable < DB > , DB : QueryMetadata < SqlTypeOf < T :: SelectExpression > > , { fn row_metadata (lookup : & mut Self :: MetadataLookup , out : & mut Vec < Option < Self :: TypeMetadata > >) { < DB as QueryMetadata < SqlTypeOf < < T as Selectable < DB > > :: SelectExpression > > > :: row_metadata (lookup , out ,) } }
};
}
