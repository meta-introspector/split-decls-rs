macro_rules! deps {
    () => {
        CloneTypeIdentifier!();
    };
}

macro_rules! CloneSuffix {
    () => {
        deps!();
        # [doc = " <clone-suffix> ::= [ . <clone-type-identifier> ] [ . <nonnegative number> ]*"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct CloneSuffix (CloneTypeIdentifier , Vec < isize >) ;
    };
}

CloneSuffix!()