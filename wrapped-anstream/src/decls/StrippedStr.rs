macro_rules! StrippedStr {
    () => {
        # [doc = " See [`strip_str`]"] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct StrippedStr < 's > { bytes : & 's [u8] , state : State , }
    };
}

StrippedStr!();