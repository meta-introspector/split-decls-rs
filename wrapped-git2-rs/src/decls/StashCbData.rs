macro_rules! deps {
    () => {
        StashCb!();
    };
}

macro_rules! StashCbData {
    () => {
        deps!();
        pub (crate) struct StashCbData < 'a > { pub callback : & 'a mut StashCb < 'a > , }
    };
}

StashCbData!()