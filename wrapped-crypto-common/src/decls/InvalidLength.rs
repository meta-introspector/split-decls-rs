macro_rules! deps {
    () => {
        InnerIvInit!();
        KeyInit!();
        KeyIvInit!();
    };
}

macro_rules! InvalidLength {
    () => {
        deps!();
        # [doc = " The error type returned when key and/or IV used in the [`KeyInit`],"] # [doc = " [`KeyIvInit`], and [`InnerIvInit`] slice-based methods had"] # [doc = " an invalid length."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub struct InvalidLength ;
    };
}

InvalidLength!();