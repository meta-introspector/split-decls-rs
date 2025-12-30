// Generated macro for try_call (macro)
macro_rules! Depcrate_calltry_call {
() => {
// Module: crate::call
// Provides: {"try_call"}
// Dependencies: {}
macro_rules ! try_call { (raw ::$ p : ident ($ ($ e : expr) ,*)) => ({ match crate :: call :: c_try (raw ::$ p ($ (crate :: call :: convert (&$ e)) ,*)) { Ok (o) => o , Err (e) => { crate :: panic :: check () ; return Err (e) } } }) }
};
}
