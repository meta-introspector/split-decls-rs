// Generated macro for impl_12 (impl)
macro_rules! Depcrate_config_testsimpl_12 {
() => {
// Module: crate::config::tests
// Provides: {"impl_12"}
// Dependencies: {}
impl Preprocessor for TestPreprocessor { fn name (& self) -> & str { "test-preprocessor" } fn run (& self , ctx : & PreprocessorContext , mut book : Book) -> Result < Book > { let mode = Mode :: from_context (ctx , self . name ()) ? ; book . push_item (BookItem :: PartTitle (format ! ("{mode:?}"))) ; Ok (book) } }
};
}
