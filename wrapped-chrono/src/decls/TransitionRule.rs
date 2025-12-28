macro_rules! deps {
    () => {
        Fixed!();
        LocalTimeType!();
        AlternateTime!();
        Transition!();
    };
}

macro_rules! TransitionRule {
    () => {
        deps!();
        # [doc = " Transition rule"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (super) enum TransitionRule { # [doc = " Fixed local time type"] Fixed (LocalTimeType) , # [doc = " Alternate local time types"] Alternate (AlternateTime) , }
    };
}

TransitionRule!()