macro_rules! deps {
    () => {
        PendingArg!();
        ArgMatches!();
    };
}

macro_rules! ArgMatcher {
    () => {
        deps!();
        # [derive (Debug , Default)] pub (crate) struct ArgMatcher { matches : ArgMatches , pending : Option < PendingArg > , }
    };
}

ArgMatcher!()