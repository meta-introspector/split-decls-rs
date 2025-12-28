macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! ValueRange {
    () => {
        deps!();
        # [doc = " Values per occurrence for an argument"] # [derive (Copy , Clone , PartialEq , Eq , Hash)] pub struct ValueRange { start_inclusive : usize , end_inclusive : usize , }
    };
}

ValueRange!()