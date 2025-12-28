macro_rules! deps {
    () => {
        RetryFailError!();
        MatchErrorKind!();
        MatchError!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl From < MatchError > for RetryFailError { fn from (merr : MatchError) -> RetryFailError { use crate :: util :: search :: MatchErrorKind :: * ; match * merr . kind () { Quit { offset , .. } => RetryFailError :: from_offset (offset) , GaveUp { offset } => RetryFailError :: from_offset (offset) , HaystackTooLong { .. } | UnsupportedAnchored { .. } => { unreachable ! ("found impossible error in meta engine: {merr}") } } } }
    };
}

impl_320!()