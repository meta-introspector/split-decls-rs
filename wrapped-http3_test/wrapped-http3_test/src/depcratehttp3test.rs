// Generated macro for Http3Test (struct)
macro_rules! DepcrateHttp3Test {
() => {
// Module: crate
// Provides: {"Http3Test"}
// Dependencies: {}
# [doc = " The main object for getting things done."] # [doc = ""] # [doc = " The factory method new() is used to set up a vector of Http3Req objects and"] # [doc = " map them to a test assertion function. The public functions are used to send"] # [doc = " requests and store response data. Internally we track some other state to"] # [doc = " make sure everything goes smoothly."] # [doc = ""] # [doc = " Many tests have similar inputs or assertions, so utility functions help"] # [doc = " cover many of the common cases like testing different status codes or"] # [doc = " checking that a response body is echoed back."] pub struct Http3Test { endpoint : url :: Url , reqs : Vec < Http3Req > , stream_data : Option < Vec < ArbitraryStreamData > > , assert : Http3Assert , issued_reqs : HashMap < u64 , usize > , concurrent : bool , current_idx : usize , }
};
}
