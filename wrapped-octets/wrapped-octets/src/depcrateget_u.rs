// Generated macro for get_u (macro)
macro_rules! Depcrateget_u {
() => {
// Module: crate
// Provides: {"get_u"}
// Dependencies: {}
macro_rules ! get_u { ($ b : expr , $ ty : ty , $ len : expr) => { { let out = peek_u ! ($ b , $ ty , $ len) ; $ b . off += $ len ; out } } ; }
};
}
