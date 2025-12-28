macro_rules! deps {
    () => {
        Fixint!();
        Varint!();
        Configuration!();
    };
}

macro_rules! IntEncoding {
    () => {
        deps!();
        # [doc = " Integer Encoding of a `Configuration`."] # [derive (PartialEq , Eq)] # [non_exhaustive] pub enum IntEncoding { # [doc = " Fixed Integer Encoding, see `Fixint`."] Fixed , # [doc = " Variable Integer Encoding, see `Varint`."] Variable , }
    };
}

IntEncoding!()