// Generated macro for Point (struct)
macro_rules! Depcrate_ecPoint {
() => {
// Module: crate::ec
// Provides: {"Point"}
// Dependencies: {}
# [doc = " Point is a valid, finite point on some curve."] pub (crate) struct Point { group : * const bssl_sys :: EC_GROUP , point : * mut bssl_sys :: EC_POINT , }
};
}
