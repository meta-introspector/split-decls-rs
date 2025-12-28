macro_rules! deps {
    () => {
        StripStr!();
    };
}

macro_rules! StripStrIter {
    () => {
        deps!();
        # [doc = " See [`StripStr`]"] # [derive (Debug , PartialEq , Eq)] pub struct StripStrIter < 's > { bytes : & 's [u8] , state : & 's mut State , }
    };
}

StripStrIter!()