// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " An actor execution context."] pub struct Context < A > where A : Actor < Context = Context < A > > , { parts : ContextParts < A > , mb : Option < Mailbox < A > > , }
};
}
