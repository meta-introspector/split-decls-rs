macro_rules! ForEachCb {
    () => {
        pub type ForEachCb < 'a > = dyn FnMut (& [u8]) -> bool + 'a ;
    };
}

ForEachCb!();