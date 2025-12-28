macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl Clone for TestDB { fn clone (& self) -> Self { Self { storage : self . storage . clone () , files : self . files . clone () , crates_map : self . crates_map . clone () , events : self . events . clone () , nonce : Nonce :: new () , } } }
    };
}

impl_587!()