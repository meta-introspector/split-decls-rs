// Generated macro for Checkout (struct)
macro_rules! Depcrate_client_legacy_poolCheckout {
() => {
// Module: crate::client::legacy::pool
// Provides: {"Checkout"}
// Dependencies: {}
# [allow (missing_debug_implementations)] pub struct Checkout < T , K : Key > { key : K , pool : Pool < T , K > , waiter : Option < oneshot :: Receiver < T > > , }
};
}
