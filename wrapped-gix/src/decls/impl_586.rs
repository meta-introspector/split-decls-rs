macro_rules! deps {
    () => {
        Branch!();
        Merge!();
        RemoteName!();
        Tree!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl Branch { # [doc = " The `branch.<name>.merge` key."] pub const MERGE : Merge = Merge :: new_with_validate ("merge" , & crate :: config :: Tree :: BRANCH , validate :: FullNameRef) . with_subsection_requirement (NAME_PARAMETER) ; # [doc = " The `branch.<name>.pushRemote` key."] pub const PUSH_REMOTE : keys :: RemoteName = keys :: RemoteName :: new_remote_name ("pushRemote" , & crate :: config :: Tree :: BRANCH) . with_subsection_requirement (NAME_PARAMETER) ; # [doc = " The `branch.<name>.remote` key."] pub const REMOTE : keys :: RemoteName = keys :: RemoteName :: new_remote_name ("remote" , & crate :: config :: Tree :: BRANCH) . with_subsection_requirement (NAME_PARAMETER) ; }
    };
}

impl_586!()