macro_rules! deps {
    () => {
        VersionVec!();
        State!();
        Access!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl State { pub (crate) fn last_dependent_access (& self) -> Option < & Access > { self . last_access . as_ref () } pub (crate) fn set_last_access (& mut self , path_id : usize , version : & VersionVec) { Access :: set_or_create (& mut self . last_access , path_id , version) } }
    };
}

impl_132!();