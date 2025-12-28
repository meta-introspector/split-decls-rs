macro_rules! deps {
    () => {
        BigEndian!();
        LittleEndian!();
        Configuration!();
    };
}

macro_rules! Endianness {
    () => {
        deps!();
        # [doc = " Endianness of a `Configuration`."] # [derive (PartialEq , Eq)] # [non_exhaustive] pub enum Endianness { # [doc = " Little Endian encoding, see `LittleEndian`."] Little , # [doc = " Big Endian encoding, see `BigEndian`."] Big , }
    };
}

Endianness!()