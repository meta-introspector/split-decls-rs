macro_rules! deps {
    () => {
        Mac!();
    };
}

macro_rules! MacError {
    () => {
        deps!();
        # [doc = " Error type for when the [`Output`] of a [`Mac`]"] # [doc = " is not equal to the expected value."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq)] pub struct MacError ;
    };
}

MacError!();