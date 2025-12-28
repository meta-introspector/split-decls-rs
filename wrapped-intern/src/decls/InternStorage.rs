macro_rules! deps {
    () => {
        InternMap!();
    };
}

macro_rules! InternStorage {
    () => {
        deps!();
        pub struct InternStorage < T : ? Sized > { map : OnceLock < InternMap < T > > , }
    };
}

InternStorage!()