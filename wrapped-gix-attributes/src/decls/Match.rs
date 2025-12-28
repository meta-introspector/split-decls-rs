macro_rules! deps {
    () => {
        MatchLocation!();
        MatchKind!();
        AssignmentRef!();
    };
}

macro_rules! Match {
    () => {
        deps!();
        # [doc = " Describes a matching pattern with"] # [derive (Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct Match < 'a > { # [doc = " The glob pattern itself, like `/target/*`."] pub pattern : & 'a gix_glob :: Pattern , # [doc = " The key=value pair of the attribute that matched at the pattern. There can be multiple matches per pattern."] pub assignment : AssignmentRef < 'a > , # [doc = " Additional information about the kind of match."] pub kind : MatchKind , # [doc = " Information about the location of the match."] pub location : MatchLocation < 'a > , }
    };
}

Match!();