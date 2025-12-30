// Generated macro for Context (struct)
macro_rules! Depcrate_pack_verifyContext {
() => {
// Module: crate::pack::verify
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A general purpose context for many operations provided here"] pub struct Context < 'a , W1 : io :: Write , W2 : io :: Write > { # [doc = " If set, provide statistics to `out` in the given format"] pub output_statistics : Option < OutputFormat > , # [doc = " A stream to which to output operation results"] pub out : W1 , # [doc = " A stream to which to errors"] pub err : W2 , # [doc = " If set, don't use more than this amount of threads."] # [doc = " Otherwise, usually use as many threads as there are logical cores."] # [doc = " A value of 0 is interpreted as no-limit"] pub thread_limit : Option < usize > , pub mode : index :: verify :: Mode , pub algorithm : Algorithm , pub should_interrupt : & 'a AtomicBool , pub object_hash : gix :: hash :: Kind , }
};
}
