macro_rules! DefUse {
    () => {
        # [derive (Eq , PartialEq , Clone)] pub (crate) enum DefUse { Def , Use , Drop , }
    };
}

DefUse!();