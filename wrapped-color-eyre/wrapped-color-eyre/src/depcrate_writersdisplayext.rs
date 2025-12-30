// Generated macro for DisplayExt (trait)
macro_rules! Depcrate_writersDisplayExt {
() => {
// Module: crate::writers
// Provides: {"DisplayExt"}
// Dependencies: {}
# [cfg (feature = "issue-url")] pub (crate) trait DisplayExt : Sized + Display { fn with_header < H : Display > (self , header : H) -> Header < Self , H > ; fn with_footer < F : Display > (self , footer : F) -> Footer < Self , F > ; }
};
}
