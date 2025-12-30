// Generated macro for impl_45 (impl)
macro_rules! Depcrate_ffidispimpl_45 {
() => {
// Module: crate::ffidisp
// Provides: {"impl_45"}
// Dependencies: {}
impl From < Message > for ConnectionItem { fn from (m : Message) -> Self { let mtype = m . msg_type () ; match mtype { MessageType :: Signal => ConnectionItem :: Signal (m) , MessageType :: MethodReturn => ConnectionItem :: MethodReturn (m) , MessageType :: Error => ConnectionItem :: MethodReturn (m) , MessageType :: MethodCall => ConnectionItem :: MethodCall (m) , } } }
};
}
