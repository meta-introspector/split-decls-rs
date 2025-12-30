// Generated macro for impl_249 (impl)
macro_rules! Depcrate_mailboximpl_249 {
() => {
// Module: crate::mailbox
// Provides: {"impl_249"}
// Dependencies: {}
impl < A > Mailbox < A > where A : Actor , A :: Context : AsyncContext < A > , { # [inline] pub fn new (msgs : AddressReceiver < A >) -> Self { Self { msgs } } pub fn capacity (& self) -> usize { self . msgs . capacity () } pub fn set_capacity (& mut self , cap : usize) { self . msgs . set_capacity (cap) ; } # [inline] pub fn connected (& self) -> bool { self . msgs . connected () } pub fn address (& self) -> Addr < A > { Addr :: new (self . msgs . sender ()) } pub fn sender_producer (& self) -> AddressSenderProducer < A > { self . msgs . sender_producer () } pub fn poll (& mut self , act : & mut A , ctx : & mut A :: Context , task : & mut task :: Context < '_ >) { # [cfg (feature = "mailbox_assert")] let mut n_polls = 0u16 ; while ! ctx . waiting () { match Pin :: new (& mut self . msgs) . poll_next (task) { Poll :: Ready (Some (mut msg)) => { msg . handle (act , ctx) ; # [cfg (feature = "mailbox_assert")] { n_polls += 1 ; assert ! (n_polls < 256u16 , "Too many messages are being processed. Use Self::Context::notify() instead of direct use of address") ; } } Poll :: Ready (None) | Poll :: Pending => return , } } } }
};
}
