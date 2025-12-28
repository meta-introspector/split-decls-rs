macro_rules! deps {
    () => {
        Utf8Parser!();
        StripBytes!();
    };
}

macro_rules! StripBytesIter {
    () => {
        deps!();
        # [doc = " See [`StripBytes`]"] # [derive (Debug , PartialEq , Eq)] pub struct StripBytesIter < 's > { bytes : & 's [u8] , state : & 's mut State , utf8parser : & 's mut Utf8Parser , }
    };
}

StripBytesIter!();