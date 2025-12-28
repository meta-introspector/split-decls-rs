macro_rules! deps {
    () => {
        Stage!();
    };
}

macro_rules! ResolvePath {
    () => {
        deps!();
        # [allow (dead_code)] # [derive (Clone)] pub struct ResolvePath { # [doc = " relative to the root of the repository, or what would be stored in the index"] name : BString , # [doc = " 0 = ancestor/common, 1 = ours, 2 = theirs"] stages : [Option < Stage > ; 3] , }
    };
}

ResolvePath!();