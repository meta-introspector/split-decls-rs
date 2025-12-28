macro_rules! SplitRange {
    () => {
        # [doc = " A tagged range indicating how it was derived from a pair of ranges."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum SplitRange { Old (Utf8Range) , New (Utf8Range) , Both (Utf8Range) , }
    };
}

SplitRange!();