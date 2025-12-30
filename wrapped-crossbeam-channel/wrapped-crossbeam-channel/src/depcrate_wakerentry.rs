// Generated macro for Entry (struct)
macro_rules! Depcrate_wakerEntry {
() => {
// Module: crate::waker
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " Represents a thread blocked on a specific channel operation."] pub (crate) struct Entry { # [doc = " The operation."] pub (crate) oper : Operation , # [doc = " Optional packet."] pub (crate) packet : * mut () , # [doc = " Context associated with the thread owning this operation."] pub (crate) cx : Context , }
};
}
