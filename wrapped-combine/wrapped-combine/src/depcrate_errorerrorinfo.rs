// Generated macro for ErrorInfo (trait)
macro_rules! Depcrate_errorErrorInfo {
() => {
// Module: crate::error
// Provides: {"ErrorInfo"}
// Dependencies: {}
# [doc = " Trait for types which can be used to construct error information."] # [doc = ""] # [doc = " To call functions expecting this trait, use the wrapper types defined in this module"] # [doc = " `Token`, `Range`, `Format` or `Static`/`&'static str`"] pub trait ErrorInfo < 's , T , R > { type Format : fmt :: Display ; # [allow (clippy :: wrong_self_convention)] fn into_info (& 's self) -> Info < T , R , Self :: Format > ; }
};
}
