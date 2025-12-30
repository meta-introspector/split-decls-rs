// Generated macro for Builder (struct)
macro_rules! Depcrate_util_prefilterBuilder {
() => {
// Module: crate::util::prefilter
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder for constructing the best possible prefilter. When constructed,"] # [doc = " this builder will heuristically select the best prefilter it can build,"] # [doc = " if any, and discard the rest."] # [derive (Debug)] pub (crate) struct Builder { count : usize , ascii_case_insensitive : bool , start_bytes : StartBytesBuilder , rare_bytes : RareBytesBuilder , memmem : MemmemBuilder , packed : Option < packed :: Builder > , enabled : bool , }
};
}
