// Generated macro for Seq (struct)
macro_rules! Depcrate_se_contentSeq {
() => {
// Module: crate::se::content
// Provides: {"Seq"}
// Dependencies: {}
# [doc = " Helper struct which remembers the classification of the last serialized element"] # [doc = " and reports it when the sequence ends"] pub struct Seq < 'w , 'k , W : Write > { ser : ContentSerializer < 'w , 'k , W > , # [doc = " Classification of the result of the last serialized element."] last : WriteResult , }
};
}
