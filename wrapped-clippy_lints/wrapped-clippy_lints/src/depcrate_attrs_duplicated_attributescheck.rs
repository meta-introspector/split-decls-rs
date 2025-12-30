// Generated macro for check (function)
macro_rules! Depcrate_attrs_duplicated_attributescheck {
() => {
// Module: crate::attrs::duplicated_attributes
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & EarlyContext < '_ > , attrs : & [Attribute]) { let mut attr_paths = FxHashMap :: default () ; for attr in attrs { if let Some (meta) = attr . meta () { check_duplicated_attr (cx , & meta , & mut attr_paths , & mut Vec :: new ()) ; } } }
};
}
