// Generated macro for impl_86 (impl)
macro_rules! Depcrate_parseimpl_86 {
() => {
// Module: crate::parse
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a > BrokenLink < 'a > { # [doc = " Moves the link into version with a static lifetime."] # [doc = ""] # [doc = " The `reference` member is cloned to a Boxed or Inline version."] pub fn into_static (self) -> BrokenLink < 'static > { BrokenLink { span : self . span . clone () , link_type : self . link_type , reference : self . reference . into_string () . into () , } } }
};
}
