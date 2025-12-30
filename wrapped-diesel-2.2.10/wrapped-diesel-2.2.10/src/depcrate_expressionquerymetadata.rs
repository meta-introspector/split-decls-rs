// Generated macro for QueryMetadata (trait)
macro_rules! Depcrate_expressionQueryMetadata {
() => {
// Module: crate::expression
// Provides: {"QueryMetadata"}
// Dependencies: {}
# [doc = " A helper to translate type level sql type information into"] # [doc = " runtime type information for specific queries"] # [doc = ""] # [doc = " If you do not implement a custom backend implementation"] # [doc = " this trait is likely not relevant for you."] pub trait QueryMetadata < T > : Backend { # [doc = " The exact return value of this function is considered to be a"] # [doc = " backend specific implementation detail. You should not rely on those"] # [doc = " values if you not own the corresponding backend"] fn row_metadata (lookup : & mut Self :: MetadataLookup , out : & mut Vec < Option < Self :: TypeMetadata > >) ; }
};
}
