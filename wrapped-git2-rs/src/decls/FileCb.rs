macro_rules! deps {
    () => {
        DiffDelta!();
    };
}

macro_rules! FileCb {
    () => {
        deps!();
        pub type FileCb < 'a > = dyn FnMut (DiffDelta < '_ > , f32) -> bool + 'a ;
    };
}

FileCb!()