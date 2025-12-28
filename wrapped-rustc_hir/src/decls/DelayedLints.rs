macro_rules! deps {
    () => {
        DelayedLint!();
    };
}

macro_rules! DelayedLints {
    () => {
        deps!();
        # [derive (Debug)] pub struct DelayedLints { pub lints : Box < [DelayedLint] > , pub opt_hash : Option < Fingerprint > , }
    };
}

DelayedLints!();