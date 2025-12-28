macro_rules! Space {
    () => {
        pub enum Space < T > { Available (T) , Full (usize) , }
    };
}

Space!()