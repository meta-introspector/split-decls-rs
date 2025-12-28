macro_rules! deps {
    () => {
        StreamGroup!();
    };
}

macro_rules! Key {
    () => {
        deps!();
        # [doc = " A key used to index into the `StreamGroup` type."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Key (usize) ;
    };
}

Key!()