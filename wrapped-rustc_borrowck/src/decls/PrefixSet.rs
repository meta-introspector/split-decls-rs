macro_rules! PrefixSet {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub (super) enum PrefixSet { # [doc = " Doesn't stop until it returns the base case (a Local or"] # [doc = " Static prefix)."] All , # [doc = " Stops at any dereference."] Shallow , }
    };
}

PrefixSet!();