// Generated macro for IndexObjectFn (type)
macro_rules! Depcrate_eol_convert_to_gitIndexObjectFn {
() => {
// Module: crate::eol::convert_to_git
// Provides: {"IndexObjectFn"}
// Dependencies: {}
# [doc = " A function that writes a buffer like `fn(&mut buf)` with by tes of an object in the index that is the one that should be converted."] pub type IndexObjectFn < 'a > = dyn FnMut (& mut Vec < u8 >) -> Result < Option < () > , Box < dyn std :: error :: Error + Send + Sync > > + 'a ;
};
}
