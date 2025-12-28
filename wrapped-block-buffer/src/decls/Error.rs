macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Block buffer error."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub struct Error ;
    };
}

Error!()