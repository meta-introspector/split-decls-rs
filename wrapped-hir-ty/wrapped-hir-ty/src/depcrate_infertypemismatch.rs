// Generated macro for TypeMismatch (struct)
macro_rules! Depcrate_inferTypeMismatch {
() => {
// Module: crate::infer
// Provides: {"TypeMismatch"}
// Dependencies: {}
# [doc = " A mismatch between an expected and an inferred type."] # [derive (Clone , PartialEq , Eq , Debug , Hash)] pub struct TypeMismatch < 'db > { pub expected : Ty < 'db > , pub actual : Ty < 'db > , }
};
}
