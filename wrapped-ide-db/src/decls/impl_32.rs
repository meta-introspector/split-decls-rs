macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Clone for RootDatabase { fn clone (& self) -> Self { Self { storage : self . storage . clone () , files : self . files . clone () , crates_map : self . crates_map . clone () , nonce : Nonce :: new () , } } }
    };
}

impl_32!()