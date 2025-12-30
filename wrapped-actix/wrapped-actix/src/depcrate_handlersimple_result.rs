// Generated macro for SIMPLE_RESULT (macro)
macro_rules! Depcrate_handlerSIMPLE_RESULT {
() => {
// Module: crate::handler
// Provides: {"SIMPLE_RESULT"}
// Dependencies: {}
macro_rules ! SIMPLE_RESULT { ($ type : ty) => { impl < A , M > MessageResponse < A , M > for $ type where A : Actor , M : Message < Result = $ type >, { fn handle (self , _ : & mut A :: Context , tx : Option < OneshotSender <$ type >>) { tx . send (self) } } } ; }
};
}
