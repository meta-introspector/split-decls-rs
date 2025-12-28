macro_rules! deps {
    () => {
        PatternID!();
        GroupInfo!();
    };
}

macro_rules! GroupInfoErrorKind {
    () => {
        deps!();
        # [doc = " The kind of error that occurs when building a `GroupInfo` fails."] # [doc = ""] # [doc = " We keep this un-exported because it's not clear how useful it is to"] # [doc = " export it."] # [derive (Clone , Debug)] enum GroupInfoErrorKind { # [doc = " This occurs when too many patterns have been added. i.e., It would"] # [doc = " otherwise overflow a `PatternID`."] TooManyPatterns { err : PatternIDError } , # [doc = " This occurs when too many capturing groups have been added for a"] # [doc = " particular pattern."] TooManyGroups { # [doc = " The ID of the pattern that had too many groups."] pattern : PatternID , # [doc = " The minimum number of groups that the caller has tried to add for"] # [doc = " a pattern."] minimum : usize , } , # [doc = " An error that occurs when a pattern has no capture groups. Either the"] # [doc = " group info must be empty, or all patterns must have at least one group"] # [doc = " (corresponding to the unnamed group for the entire pattern)."] MissingGroups { # [doc = " The ID of the pattern that had no capturing groups."] pattern : PatternID , } , # [doc = " An error that occurs when one tries to provide a name for the capture"] # [doc = " group at index 0. This capturing group must currently always be"] # [doc = " unnamed."] FirstMustBeUnnamed { # [doc = " The ID of the pattern that was found to have a named first"] # [doc = " capturing group."] pattern : PatternID , } , # [doc = " An error that occurs when duplicate capture group names for the same"] # [doc = " pattern are added."] # [doc = ""] # [doc = " NOTE: At time of writing, this error can never occur if you're using"] # [doc = " regex-syntax, since the parser itself will reject patterns with"] # [doc = " duplicate capture group names. This error can only occur when the"] # [doc = " builder is used to hand construct NFAs."] Duplicate { # [doc = " The pattern in which the duplicate capture group name was found."] pattern : PatternID , # [doc = " The duplicate name."] name : String , } , }
    };
}

GroupInfoErrorKind!();