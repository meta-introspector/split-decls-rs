macro_rules! MacHeader {
    () => {
        pub enum MacHeader < 'a > { Path (& 'a ast :: Path) , Keyword (& 'static str) , }
    };
}

MacHeader!();