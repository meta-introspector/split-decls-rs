macro_rules! RawArgs {
    () => {
        # [doc = " Command-line arguments"] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct RawArgs { items : Vec < OsString > , }
    };
}

RawArgs!()