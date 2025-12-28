macro_rules! deps {
    () => {
        Tree!();
        Url!();
        Any!();
    };
}

macro_rules! impl_753 {
    () => {
        deps!();
        impl Url { # [doc = " The `url.<base>.insteadOf` key"] pub const INSTEAD_OF : keys :: Any = keys :: Any :: new ("insteadOf" , & config :: Tree :: URL) . with_subsection_requirement (BASE_PARAMETER) ; # [doc = " The `url.<base>.pushInsteadOf` key"] pub const PUSH_INSTEAD_OF : keys :: Any = keys :: Any :: new ("pushInsteadOf" , & config :: Tree :: URL) . with_subsection_requirement (BASE_PARAMETER) ; }
    };
}

impl_753!()