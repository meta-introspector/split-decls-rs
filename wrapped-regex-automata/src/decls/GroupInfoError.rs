macro_rules! deps {
    () => {
        GroupInfo!();
        GroupInfoErrorKind!();
    };
}

macro_rules! GroupInfoError {
    () => {
        deps!();
        # [doc = " An error that may occur when building a `GroupInfo`."] # [doc = ""] # [doc = " Building a `GroupInfo` does a variety of checks to make sure the"] # [doc = " capturing groups satisfy a number of invariants. This includes, but is not"] # [doc = " limited to, ensuring that the first capturing group is unnamed and that"] # [doc = " there are no duplicate capture groups for a specific pattern."] # [derive (Clone , Debug)] pub struct GroupInfoError { kind : GroupInfoErrorKind , }
    };
}

GroupInfoError!()