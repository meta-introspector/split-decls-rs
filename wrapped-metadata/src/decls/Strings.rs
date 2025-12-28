macro_rules! Strings {
    () => {
        pub struct Strings { map : HashMap < String , id :: StringId > , stream : Vec < u8 > , }
    };
}

Strings!()