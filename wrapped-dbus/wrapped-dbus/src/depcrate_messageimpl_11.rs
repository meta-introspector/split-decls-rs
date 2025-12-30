// Generated macro for impl_11 (impl)
macro_rules! Depcrate_messageimpl_11 {
() => {
// Module: crate::message
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for MessageType { type Error = () ; fn try_from (value : & 'a str) -> Result < Self , < crate :: message :: MessageType as TryFrom < & 'a str > > :: Error > { match value { "error" => Ok (MessageType :: Error) , "method_call" => Ok (MessageType :: MethodCall) , "method_return" => Ok (MessageType :: MethodReturn) , "signal" => Ok (MessageType :: Signal) , _ => Err (()) } } }
};
}
