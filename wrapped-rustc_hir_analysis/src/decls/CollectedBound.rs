macro_rules! CollectedBound {
    () => {
        # [derive (Debug , Default)] struct CollectedBound { # [doc = " `Trait`"] positive : bool , # [doc = " `?Trait`"] maybe : bool , # [doc = " `!Trait`"] negative : bool , }
    };
}

CollectedBound!();