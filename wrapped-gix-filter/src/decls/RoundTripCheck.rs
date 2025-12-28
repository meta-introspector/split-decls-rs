macro_rules! RoundTripCheck {
    () => {
        # [doc = " Whether or not to perform round-trip checks."] # [derive (Debug , Copy , Clone)] pub enum RoundTripCheck { # [doc = " Assure that we can losslessly convert the UTF-8 result back to the original encoding or fail with an error."] Fail , # [doc = " Do not check if the encoding is round-trippable."] Skip , }
    };
}

RoundTripCheck!()