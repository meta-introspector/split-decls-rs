macro_rules! deps {
    () => {
        AlternateTime!();
        Transition!();
        LocalTimeType!();
        Fixed!();
    };
}

macro_rules! TransitionRule {
    () => {
        deps!();
        # [doc = " Transition rule"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (super) enum TransitionRule { # [doc = " Fixed local time type"] Fixed (LocalTimeType) , # [doc = " Alternate local time types"] Alternate (AlternateTime) , }
    };
}

TransitionRule!();