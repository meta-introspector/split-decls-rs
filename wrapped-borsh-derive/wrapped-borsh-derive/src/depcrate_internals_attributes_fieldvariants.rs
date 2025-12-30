// Generated macro for Variants (enum)
macro_rules! Depcrate_internals_attributes_fieldVariants {
() => {
// Module: crate::internals::attributes::field
// Provides: {"Variants"}
// Dependencies: {}
enum Variants { Bounds (bounds :: Bounds) , SerializeWith (syn :: ExprPath) , DeserializeWith (syn :: ExprPath) , Skip (()) , # [cfg (feature = "schema")] Schema (schema :: Attributes) , }
};
}
