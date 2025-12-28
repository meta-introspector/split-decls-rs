macro_rules! deps {
    () => {
        UnicodeWordBoundaryError!();
    };
}

macro_rules! impl_696 {
    () => {
        deps!();
        impl UnicodeWordBoundaryError { # [cfg (not (feature = "unicode-word-boundary"))] pub (crate) fn new () -> UnicodeWordBoundaryError { UnicodeWordBoundaryError (()) } # [doc = " Returns an error if and only if Unicode word boundary data is"] # [doc = " unavailable."] pub fn check () -> Result < () , UnicodeWordBoundaryError > { is_word_char :: check () } }
    };
}

impl_696!();