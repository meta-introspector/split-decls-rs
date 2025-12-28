macro_rules! DebugByte {
    () => {
        # [doc = " Provides a convenient `Debug` implementation for a `u8`."] # [doc = ""] # [doc = " The `Debug` impl treats the byte as an ASCII, and emits a human readable"] # [doc = " representation of it. If the byte isn't ASCII, then it's emitted as a hex"] # [doc = " escape sequence."] # [derive (Clone , Copy)] pub struct DebugByte (pub u8) ;
    };
}

DebugByte!();