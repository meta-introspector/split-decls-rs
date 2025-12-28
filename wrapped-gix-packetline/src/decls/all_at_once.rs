macro_rules! deps {
    () => {
        Error!();
        PacketLineRef!();
        Stream!();
    };
}

macro_rules! all_at_once {
    () => {
        deps!();
        # [doc = " Decode an entire packet line from data or fail."] # [doc = ""] # [doc = " Note that failure also happens if there is not enough data to parse a complete packet line, as opposed to [`streaming()`] decoding"] # [doc = " succeeds in that case, stating how much more bytes are required."] pub fn all_at_once (data : & [u8]) -> Result < PacketLineRef < '_ > , Error > { match streaming (data) ? { Stream :: Complete { line , .. } => Ok (line) , Stream :: Incomplete { bytes_needed } => Err (Error :: NotEnoughData { bytes_needed }) , } }
    };
}

all_at_once!();