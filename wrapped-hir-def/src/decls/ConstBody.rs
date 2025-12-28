macro_rules! deps {
    () => {
        SimpleBody!();
    };
}

macro_rules! ConstBody {
    () => {
        deps!();
        pub type ConstBody = SimpleBody ;
    };
}

ConstBody!()