macro_rules! MatchFailureReason {
    () => {
        # [derive (Debug)] pub (crate) struct MatchFailureReason { pub (crate) reason : String , }
    };
}

MatchFailureReason!();