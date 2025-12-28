macro_rules! deps {
    () => {
        Error!();
        PacketLineRef!();
    };
}

macro_rules! to_data_line {
    () => {
        deps!();
        # [doc = " Obtain a `PacketLine` from `data` after assuring `data` is small enough to fit."] pub fn to_data_line (data : & [u8]) -> Result < PacketLineRef < '_ > , Error > { if data . len () > MAX_LINE_LEN { return Err (Error :: DataLengthLimitExceeded { length_in_bytes : data . len () , }) ; } Ok (PacketLineRef :: Data (data)) }
    };
}

to_data_line!()