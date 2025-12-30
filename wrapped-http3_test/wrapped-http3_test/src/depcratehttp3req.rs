// Generated macro for Http3Req (struct)
macro_rules! DepcrateHttp3Req {
() => {
// Module: crate
// Provides: {"Http3Req"}
// Dependencies: {}
# [doc = " Stores the request, the expected response headers, and the actual response."] # [doc = ""] # [doc = " The assert_headers! macro is provided for convenience to validate the"] # [doc = " received headers match the expected headers."] # [derive (Clone)] pub struct Http3Req { pub url : url :: Url , pub hdrs : Vec < Header > , pub body : Option < Vec < u8 > > , pub expect_resp_hdrs : Option < Vec < Header > > , pub resp_hdrs : Vec < Header > , pub resp_body : Vec < u8 > , pub reset_stream_code : Option < u64 > , }
};
}
