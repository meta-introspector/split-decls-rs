macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! hex_to {
    () => {
        deps!();
        # [deprecated (since = "0.3.0" , note = "please use `hex_encode` instead")] pub fn hex_to (src : & [u8] , dst : & mut [u8]) -> Result < () , Error > { hex_encode (src , dst) . map (| _ | ()) }
    };
}

hex_to!()