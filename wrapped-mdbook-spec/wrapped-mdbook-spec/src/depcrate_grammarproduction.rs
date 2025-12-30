// Generated macro for Production (struct)
macro_rules! Depcrate_grammarProduction {
() => {
// Module: crate::grammar
// Provides: {"Production"}
// Dependencies: {}
# [derive (Debug)] pub struct Production { name : String , # [doc = " Comments and breaks that precede the production name."] comments : Vec < Expression > , # [doc = " Category is from the markdown lang string, and defines how it is"] # [doc = " grouped and organized on the summary page."] category : String , expression : Expression , # [doc = " The path to the chapter where this is defined."] path : PathBuf , is_root : bool , }
};
}
