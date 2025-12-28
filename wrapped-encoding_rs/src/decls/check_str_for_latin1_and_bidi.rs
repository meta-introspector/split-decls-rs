macro_rules! deps {
    () => {
        Latin1Bidi!();
    };
}

macro_rules! check_str_for_latin1_and_bidi {
    () => {
        deps!();
        # [doc = " Checks whether a valid UTF-8 buffer contains code points"] # [doc = " that trigger right-to-left processing or is all-Latin1."] # [doc = ""] # [doc = " Possibly more efficient than performing the checks separately."] # [doc = ""] # [doc = " Returns `Latin1Bidi::Latin1` if `is_str_latin1()` would return `true`."] # [doc = " Otherwise, returns `Latin1Bidi::Bidi` if `is_str_bidi()` would return"] # [doc = " `true`. Otherwise, returns `Latin1Bidi::LeftToRight`."] pub fn check_str_for_latin1_and_bidi (buffer : & str) -> Latin1Bidi { if let Some (offset) = is_str_latin1_impl (buffer) { if is_str_bidi (& buffer [offset ..]) { Latin1Bidi :: Bidi } else { Latin1Bidi :: LeftToRight } } else { Latin1Bidi :: Latin1 } }
    };
}

check_str_for_latin1_and_bidi!();