// Generated macro for Local (enum)
macro_rules! Depcrate_unwrapLocal {
() => {
// Module: crate::unwrap
// Provides: {"Local"}
// Dependencies: {}
# [derive (Clone , Debug , Eq)] enum Local { # [doc = " `x.field1.field2.field3`"] WithFieldAccess { local_id : HirId , # [doc = " The indices of the field accessed."] # [doc = ""] # [doc = " Stored last-to-first, e.g. for the example above: `[field3, field2, field1]`"] field_indices : Vec < FieldIdx > , # [doc = " The span of the whole expression"] span : Span , } , # [doc = " `x`"] Pure { local_id : HirId } , }
};
}
