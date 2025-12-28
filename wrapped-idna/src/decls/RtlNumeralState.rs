macro_rules! RtlNumeralState {
    () => {
        # [doc = " For keeping track of what kind of numerals have been"] # [doc = " seen in an RTL label."] # [derive (Debug , PartialEq , Eq)] enum RtlNumeralState { Undecided , European , Arabic , }
    };
}

RtlNumeralState!()