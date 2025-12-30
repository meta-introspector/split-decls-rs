// Generated macro for impl_29 (impl)
macro_rules! Depcrate_tree_rootimpl_29 {
() => {
// Module: crate::tree::root
// Provides: {"impl_29"}
// Dependencies: {}
impl crate :: Root for Arc < Root > { type WeakRoot = Weak < Root > ; fn messages_capacity (& self) -> usize { self . deref () . messages_capacity () } fn num_tasks (& self) -> usize { self . deref () . num_tasks () } fn sorted_snapshot (& self , out : & mut Vec < (Key , Task) >) { self . deref () . sorted_snapshot (out) } fn copy_messages (& self , out : & mut Vec < Message >) { self . deref () . copy_messages (out) } fn copy_new_messages (& self , out : & mut Vec < Message > , prev : Option < MessageCopyState >) -> MessageCopyState { self . deref () . copy_new_messages (out , prev) } fn downgrade (& self) -> Self :: WeakRoot { Arc :: downgrade (self) } }
};
}
