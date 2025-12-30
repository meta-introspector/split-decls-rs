// Generated macro for assert_future (function)
macro_rules! Depcrateassert_future {
() => {
// Module: crate
// Provides: {"assert_future"}
// Dependencies: {}
fn assert_future < A , B , F > (t : F) -> F where F : Future < Item = A , Error = B > , A : Send + 'static , B : Send + 'static , { t }
};
}
