macro_rules! Build {
    () => {
        # [derive (Serialize , Debug)] pub struct Build { pub rustflags : Vec < String > , }
    };
}

Build!()