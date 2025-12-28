macro_rules! Response {
    () => {
        # [derive (Debug)] pub struct Response < 'a > (& 'a http :: Response < () >) ;
    };
}

Response!();