// Generated macro for print_crate (function)
macro_rules! Depcrate_pprust_stateprint_crate {
() => {
// Module: crate::pprust::state
// Provides: {"print_crate"}
// Dependencies: {}
# [doc = " Requires you to pass an input filename and reader so that"] # [doc = " it can scan the input text for comments to copy forward."] pub fn print_crate < 'a > (sm : & 'a SourceMap , krate : & ast :: Crate , filename : FileName , input : String , ann : & 'a dyn PpAnn , is_expanded : bool , edition : Edition , g : & AttrIdGenerator ,) -> String { let mut s = State { s : pp :: Printer :: new () , comments : Some (Comments :: new (sm , filename , input)) , ann , is_sdylib_interface : false , } ; print_crate_inner (& mut s , krate , is_expanded , edition , g) ; s . s . eof () }
};
}
