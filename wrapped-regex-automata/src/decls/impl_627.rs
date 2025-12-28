macro_rules! deps {
    () => {
        GroupInfoError!();
        GroupInfoErrorKind!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for GroupInfoError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { GroupInfoErrorKind :: TooManyPatterns { .. } | GroupInfoErrorKind :: TooManyGroups { .. } | GroupInfoErrorKind :: MissingGroups { .. } | GroupInfoErrorKind :: FirstMustBeUnnamed { .. } | GroupInfoErrorKind :: Duplicate { .. } => None , } } }
    };
}

impl_627!()