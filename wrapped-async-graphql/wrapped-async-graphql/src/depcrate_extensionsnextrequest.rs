// Generated macro for NextRequest (struct)
macro_rules! Depcrate_extensionsNextRequest {
() => {
// Module: crate::extensions
// Provides: {"NextRequest"}
// Dependencies: {}
# [doc = " The remainder of a extension chain for request."] pub struct NextRequest < 'a > { chain : & 'a [Arc < dyn Extension >] , request_fut : RequestFut < 'a > , }
};
}
