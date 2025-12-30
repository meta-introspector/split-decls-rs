// Generated macro for Vec4 (trait)
macro_rules! Depcrate_typesVec4 {
() => {
// Module: crate::types
// Provides: {"Vec4"}
// Dependencies: {}
# [doc = " A vector composed of four elements, which may be words or themselves vectors."] pub trait Vec4 < W > { fn extract (self , i : u32) -> W ; fn insert (self , w : W , i : u32) -> Self ; }
};
}
