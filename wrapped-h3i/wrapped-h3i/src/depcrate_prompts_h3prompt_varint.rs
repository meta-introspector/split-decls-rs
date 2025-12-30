// Generated macro for prompt_varint (function)
macro_rules! Depcrate_prompts_h3prompt_varint {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_varint"}
// Dependencies: {}
fn prompt_varint (str : & str) -> InquireResult < u64 > { let id = Text :: new (str) . with_validator (h3 :: validate_varint) . with_placeholder ("Integer <= 2^62 -1") . with_help_message (ESC_TO_RET) . prompt () ? ; Ok (id . parse :: < u64 > () . unwrap ()) }
};
}
