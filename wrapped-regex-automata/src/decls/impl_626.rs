macro_rules! deps {
    () => {
        GroupInfoErrorKind!();
        GroupInfoError!();
        PatternID!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl GroupInfoError { fn too_many_patterns (err : PatternIDError) -> GroupInfoError { GroupInfoError { kind : GroupInfoErrorKind :: TooManyPatterns { err } } } fn too_many_groups (pattern : PatternID , minimum : usize) -> GroupInfoError { GroupInfoError { kind : GroupInfoErrorKind :: TooManyGroups { pattern , minimum } , } } fn missing_groups (pattern : PatternID) -> GroupInfoError { GroupInfoError { kind : GroupInfoErrorKind :: MissingGroups { pattern } } } fn first_must_be_unnamed (pattern : PatternID) -> GroupInfoError { GroupInfoError { kind : GroupInfoErrorKind :: FirstMustBeUnnamed { pattern } , } } fn duplicate (pattern : PatternID , name : & str) -> GroupInfoError { GroupInfoError { kind : GroupInfoErrorKind :: Duplicate { pattern , name : String :: from (name) , } , } } }
    };
}

impl_626!()