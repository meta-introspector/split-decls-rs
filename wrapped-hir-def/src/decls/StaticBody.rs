macro_rules! deps {
    () => {
        SimpleBody!();
    };
}

macro_rules! StaticBody {
    () => {
        deps!();
        pub type StaticBody = SimpleBody ;
    };
}

StaticBody!();