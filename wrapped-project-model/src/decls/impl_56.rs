macro_rules! deps {
    () => {
        Build!();
        DepKind!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl DepKind { fn iter (list : & [cargo_metadata :: DepKindInfo]) -> impl Iterator < Item = Self > { let mut dep_kinds = [None ; 3] ; if list . is_empty () { dep_kinds [0] = Some (Self :: Normal) ; } for info in list { match info . kind { cargo_metadata :: DependencyKind :: Normal => dep_kinds [0] = Some (Self :: Normal) , cargo_metadata :: DependencyKind :: Development => dep_kinds [1] = Some (Self :: Dev) , cargo_metadata :: DependencyKind :: Build => dep_kinds [2] = Some (Self :: Build) , cargo_metadata :: DependencyKind :: Unknown => continue , } } dep_kinds . into_iter () . flatten () } }
    };
}

impl_56!()