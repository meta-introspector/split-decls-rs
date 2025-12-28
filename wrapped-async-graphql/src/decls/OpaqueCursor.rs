macro_rules! OpaqueCursor {
    () => {
        # [doc = " A opaque cursor that encode/decode the value to base64"] pub struct OpaqueCursor < T > (pub T) ;
    };
}

OpaqueCursor!();