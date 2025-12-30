// Generated macro for impl_520 (impl)
macro_rules! Depcrate_diffimpl_520 {
() => {
// Module: crate::diff
// Provides: {"impl_520"}
// Dependencies: {}
impl Diff < 'static > { # [doc = " Read the contents of a git patch file into a `git_diff` object."] # [doc = ""] # [doc = " The diff object produced is similar to the one that would be"] # [doc = " produced if you actually produced it computationally by comparing"] # [doc = " two trees, however there may be subtle differences. For example,"] # [doc = " a patch file likely contains abbreviated object IDs, so the"] # [doc = " object IDs parsed by this function will also be abbreviated."] pub fn from_buffer (buffer : & [u8]) -> Result < Diff < 'static > , Error > { crate :: init () ; let mut diff : * mut raw :: git_diff = std :: ptr :: null_mut () ; unsafe { try_call ! (raw :: git_diff_from_buffer (& mut diff , buffer . as_ptr () as * const c_char , buffer . len ())) ; Ok (Diff :: from_raw (diff)) } } }
};
}
