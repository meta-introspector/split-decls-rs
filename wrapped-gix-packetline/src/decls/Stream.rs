macro_rules! deps {
    () => {
        PacketLineRef!();
    };
}

macro_rules! Stream {
    () => {
        deps!();
        # [doc = " A utility return type to support incremental parsing of packet lines."] # [derive (Debug , Clone)] pub enum Stream < 'a > { # [doc = " Indicate a single packet line was parsed completely"] Complete { # [doc = " The parsed packet line"] line : PacketLineRef < 'a > , # [doc = " The amount of bytes consumed from input"] bytes_consumed : usize , } , # [doc = " A packet line could not yet be parsed due to missing bytes"] Incomplete { # [doc = " The amount of additional bytes needed for the parsing to complete"] bytes_needed : usize , } , }
    };
}

Stream!();