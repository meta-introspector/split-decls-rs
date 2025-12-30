// Generated macro for ResponseTypeItem (enum)
macro_rules! Depcrate_handlerResponseTypeItem {
() => {
// Module: crate::handler
// Provides: {"ResponseTypeItem"}
// Dependencies: {}
enum ResponseTypeItem < I > { Result (I) , Fut (Pin < Box < dyn Future < Output = I > > >) , }
};
}
