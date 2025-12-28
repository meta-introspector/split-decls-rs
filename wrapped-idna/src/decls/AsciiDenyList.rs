macro_rules! AsciiDenyList {
    () => {
        # [doc = " The ASCII deny list to be applied."] # [derive (PartialEq , Eq , Copy , Clone)] # [repr (transparent)] pub struct AsciiDenyList { bits : u128 , }
    };
}

AsciiDenyList!()