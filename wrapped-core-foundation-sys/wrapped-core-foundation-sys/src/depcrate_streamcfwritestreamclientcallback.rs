// Generated macro for CFWriteStreamClientCallBack (type)
macro_rules! Depcrate_streamCFWriteStreamClientCallBack {
() => {
// Module: crate::stream
// Provides: {"CFWriteStreamClientCallBack"}
// Dependencies: {}
pub type CFWriteStreamClientCallBack = extern "C" fn (stream : CFWriteStreamRef , _type : CFStreamEventType , clientCallBackInfo : * mut c_void ,) ;
};
}
