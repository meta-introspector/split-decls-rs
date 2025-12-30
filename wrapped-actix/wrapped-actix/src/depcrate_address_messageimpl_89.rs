// Generated macro for impl_89 (impl)
macro_rules! Depcrate_address_messageimpl_89 {
() => {
// Module: crate::address::message
// Provides: {"impl_89"}
// Dependencies: {}
impl < S , M > Future for MsgRequest < S , M > where S : Sender < M > , M : Message + Send , M :: Result : Send , { type Output = Result < M :: Result , MailboxError > ; fn poll (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Some ((sender , msg)) = this . info . take () { match sender . send (msg) { Ok (rx) => * this . rx = Some (rx) , Err (SendError :: Full (msg)) => { * this . info = Some ((sender , msg)) ; return Poll :: Pending ; } Err (SendError :: Closed (_)) => return Poll :: Ready (Err (MailboxError :: Closed)) , } } match this . rx { Some (rx) => match Pin :: new (rx) . poll (cx) { Poll :: Ready (res) => Poll :: Ready (res . map_err (| _ | MailboxError :: Closed)) , Poll :: Pending => match this . timeout . as_pin_mut () { Some (timeout) => timeout . poll (cx) . map (| _ | Err (MailboxError :: Timeout)) , None => Poll :: Pending , } , } , None => Poll :: Ready (Err (MailboxError :: Closed)) , } } }
};
}
