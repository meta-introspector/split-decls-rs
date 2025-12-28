macro_rules! CapacityError {
    () => {
        # [doc = " Error value indicating insufficient capacity"] # [derive (Clone , Copy , Eq , Ord , PartialEq , PartialOrd)] pub struct CapacityError < T = () > { element : T , }
    };
}

CapacityError!();