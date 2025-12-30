// Generated macro for mock_it (function)
macro_rules! Depcratemock_it {
() => {
// Module: crate
// Provides: {"mock_it"}
// Dependencies: {}
fn mock_it < M : Into < MockableItem > > (inputs : M) -> TokenStream { let mockable : MockableItem = inputs . into () ; let mock = MockItem :: from (mockable) ; let ts = mock . into_token_stream () ; if env :: var ("MOCKALL_DEBUG") . is_ok () { println ! ("{ts}") ; } ts }
};
}
