macro_rules! deps {
    () => {
        Configuration!();
        BigEndian!();
        LittleEndian!();
    };
}

macro_rules! Endianness {
    () => {
        deps!();
        # [doc = " Endianness of a `Configuration`."] # [derive (PartialEq , Eq)] # [non_exhaustive] pub enum Endianness { # [doc = " Little Endian encoding, see `LittleEndian`."] Little , # [doc = " Big Endian encoding, see `BigEndian`."] Big , }
    };
}

Endianness!();