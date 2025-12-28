macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! SerializeVec {
    () => {
        deps!();
        pub struct SerializeVec { vec : Vec < Value > , }
    };
}

SerializeVec!()