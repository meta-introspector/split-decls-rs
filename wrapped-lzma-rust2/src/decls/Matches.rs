macro_rules! Matches {
    () => {
        pub (crate) struct Matches { pub (crate) len : Vec < u32 > , pub (crate) dist : Vec < i32 > , pub (crate) count : u32 , }
    };
}

Matches!()