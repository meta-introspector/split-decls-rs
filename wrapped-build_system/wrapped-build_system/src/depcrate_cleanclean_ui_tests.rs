// Generated macro for clean_ui_tests (function)
macro_rules! Depcrate_cleanclean_ui_tests {
() => {
// Module: crate::clean
// Provides: {"clean_ui_tests"}
// Dependencies: {}
fn clean_ui_tests () -> Result < () , String > { let path = Path :: new (crate :: BUILD_DIR) . join ("rust/build/x86_64-unknown-linux-gnu/test/ui/") ; run_command (& [& "find" , & path , & "-name" , & "stamp" , & "-delete"] , None) ? ; Ok (()) }
};
}
