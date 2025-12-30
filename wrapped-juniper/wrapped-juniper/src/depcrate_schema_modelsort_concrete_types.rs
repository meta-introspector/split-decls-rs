// Generated macro for sort_concrete_types (function)
macro_rules! Depcrate_schema_modelsort_concrete_types {
() => {
// Module: crate::schema::model
// Provides: {"sort_concrete_types"}
// Dependencies: {}
# [doc = " Sorts the provided [`TypeType`]s in the \"type-then-name\" manner."] fn sort_concrete_types < S > (types : & mut [TypeType < S >]) { types . sort_by (| a , b | { concrete_type_sort :: by_type (a) . cmp (& concrete_type_sort :: by_type (b)) . then_with (| | concrete_type_sort :: by_name (a) . cmp (& concrete_type_sort :: by_name (b))) }) ; }
};
}
