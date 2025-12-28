macro_rules! Comments {
    () => {
        pub struct Comments < 'a > { sm : & 'a SourceMap , reversed_comments : Vec < Comment > , }
    };
}

Comments!()