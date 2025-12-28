macro_rules! Response {
    () => {
        # [doc = " Response from the test http server"] pub struct Response { pub code : u32 , pub headers : Vec < String > , pub body : Vec < u8 > , }
    };
}

Response!();