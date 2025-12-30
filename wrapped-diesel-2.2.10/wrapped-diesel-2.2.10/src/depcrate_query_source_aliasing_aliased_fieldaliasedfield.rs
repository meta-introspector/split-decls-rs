// Generated macro for AliasedField (struct)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldAliasedField {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"AliasedField"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , DieselNumericOps)] # [doc = " Represents an aliased field (column) within diesel's query builder"] # [doc = ""] # [doc = " See [`alias!`](crate::alias) for more details."] pub struct AliasedField < S , F > { pub (super) _alias_source : PhantomData < S > , pub (super) _field : F , }
};
}
