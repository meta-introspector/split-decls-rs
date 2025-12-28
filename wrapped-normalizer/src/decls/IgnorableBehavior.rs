macro_rules! IgnorableBehavior {
    () => {
        # [doc = " Treatment of the ignorable marker (0xFFFFFFFF) in data."] # [derive (Debug , PartialEq , Eq)] enum IgnorableBehavior { # [doc = " 0xFFFFFFFF in data is not supported."] Unsupported , # [doc = " Ignorables are ignored."] Ignored , # [doc = " Ignorables are treated as singleton decompositions"] # [doc = " to the REPLACEMENT CHARACTER."] ReplacementCharacter , }
    };
}

IgnorableBehavior!()