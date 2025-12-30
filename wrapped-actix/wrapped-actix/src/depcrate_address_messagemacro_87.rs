// Generated macro for macro_87 (macro)
macro_rules! Depcrate_address_messagemacro_87 {
() => {
// Module: crate::address::message
// Provides: {"macro_87"}
// Dependencies: {}
pin_project ! { # [doc = " A `Future` which represents an asynchronous message sending process."] # [must_use = "You must wait on the request otherwise the Message will not be delivered"] pub struct MsgRequest < S , M > where S : Sender < M >, M : Message , M : Send , M :: Result : Send { rx : Option < oneshot :: Receiver < M :: Result >>, info : Option < (S , M) >, # [pin] timeout : Option < Sleep >, } }
};
}
