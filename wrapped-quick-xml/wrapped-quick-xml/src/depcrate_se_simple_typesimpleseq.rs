// Generated macro for SimpleSeq (struct)
macro_rules! Depcrate_se_simple_typeSimpleSeq {
() => {
// Module: crate::se::simple_type
// Provides: {"SimpleSeq"}
// Dependencies: {}
# [doc = " Serializer for a sequence of atomic values delimited by space"] pub struct SimpleSeq < W : Write > { writer : W , target : QuoteTarget , level : QuoteLevel , # [doc = " If `true`, nothing was written yet to the `writer`"] is_empty : bool , }
};
}
