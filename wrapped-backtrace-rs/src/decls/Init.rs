macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! Init {
    () => {
        deps!();
        pub struct Init { lock : HANDLE , }
    };
}

Init!()