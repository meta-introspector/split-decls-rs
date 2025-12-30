// Generated macro for HttpProgress (struct)
macro_rules! Depcrate_struct_with_genericHttpProgress {
() => {
// Module: crate::struct_with_generic
// Provides: {"HttpProgress"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Debug , Default , PartialEq)] pub struct HttpProgress { pub Stage : HttpProgressStage , pub BytesSent : u64 , pub TotalBytesToSend : Option < IReference < u64 > > , pub BytesReceived : u64 , pub TotalBytesToReceive : Option < IReference < u64 > > , pub Retries : u32 , }
};
}
