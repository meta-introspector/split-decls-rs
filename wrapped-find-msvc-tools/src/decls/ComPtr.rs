macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! ComPtr {
    () => {
        deps!();
        pub struct ComPtr < T > (* mut T) where T : Interface ;
    };
}

ComPtr!();