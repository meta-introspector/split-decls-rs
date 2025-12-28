macro_rules! deps {
    () => {
        Utf8Parser!();
    };
}

macro_rules! StrippedBytes {
    () => {
        deps!();
        # [doc = " See [`strip_bytes`]"] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct StrippedBytes < 's > { bytes : & 's [u8] , state : State , utf8parser : Utf8Parser , }
    };
}

StrippedBytes!()