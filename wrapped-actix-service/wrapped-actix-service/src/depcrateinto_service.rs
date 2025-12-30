// Generated macro for into_service (function)
macro_rules! Depcrateinto_service {
() => {
// Module: crate
// Provides: {"into_service"}
// Dependencies: {}
# [doc = " Convert object of type `U` to a service `S`"] pub fn into_service < I , S , Req > (tp : I) -> S where I : IntoService < S , Req > , S : Service < Req > , { tp . into_service () }
};
}
