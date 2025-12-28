macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! TryEnum {
    () => {
        deps!();
        # [doc = " Enum types that implement `std::ops::Try` trait."] # [derive (Clone , Copy , Debug)] pub enum TryEnum { Result , Option , }
    };
}

TryEnum!();