// Generated macro for CFReadStreamClientCallBack (type)
macro_rules! Depcrate_streamCFReadStreamClientCallBack {
() => {
// Module: crate::stream
// Provides: {"CFReadStreamClientCallBack"}
// Dependencies: {}
pub type CFReadStreamClientCallBack = extern "C" fn (stream : CFReadStreamRef , _type : CFStreamEventType , clientCallBackInfo : * mut c_void ,) ;
};
}
