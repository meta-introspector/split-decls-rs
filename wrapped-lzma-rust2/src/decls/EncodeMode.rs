macro_rules! EncodeMode {
    () => {
        # [doc = " The mode to use when encoding."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum EncodeMode { # [doc = " Fast mode (lower quality but faster)."] Fast , # [doc = " Normal mode (higher quality but slower)."] Normal , }
    };
}

EncodeMode!();