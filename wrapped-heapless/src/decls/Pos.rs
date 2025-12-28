macro_rules! Pos {
    () => {
        # [doc (hidden)] # [derive (Clone , Copy , PartialEq)] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] pub struct Pos { nz : NonZeroU32 , }
    };
}

Pos!();