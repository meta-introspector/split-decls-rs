// Generated macro for do_mock_once (function)
macro_rules! Depcratedo_mock_once {
() => {
// Module: crate
// Provides: {"do_mock_once"}
// Dependencies: {}
fn do_mock_once (input : TokenStream) -> TokenStream { let item : MockableStruct = match syn :: parse2 (input) { Ok (mock) => mock , Err (err) => { return err . to_compile_error () ; } } ; mock_it (item) }
};
}
