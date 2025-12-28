macro_rules! PropertyEnumToValueNameLookup {
    () => {
        pub trait PropertyEnumToValueNameLookup { fn get (& self , prop : u32) -> Option < & str > ; }
    };
}

PropertyEnumToValueNameLookup!();