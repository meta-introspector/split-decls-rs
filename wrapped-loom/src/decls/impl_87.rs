macro_rules! deps {
    () => {
        Store!();
        Atomic!();
        Condvar!();
        Channel!();
        VersionVec!();
        Mutex!();
        Access!();
        Operation!();
        RwLock!();
        Arc!();
        Notify!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Store { pub (super) fn last_dependent_access (& self , operation : Operation) -> Option < & Access > { match & self . entries [operation . obj . index] { Entry :: Arc (entry) => entry . last_dependent_access (operation . action . into ()) , Entry :: Atomic (entry) => entry . last_dependent_access (operation . action . into ()) , Entry :: Mutex (entry) => entry . last_dependent_access () , Entry :: Condvar (entry) => entry . last_dependent_access () , Entry :: Notify (entry) => entry . last_dependent_access () , Entry :: RwLock (entry) => entry . last_dependent_access () , Entry :: Channel (entry) => entry . last_dependent_access (operation . action . into ()) , obj => panic ! ("object is not branchable {:?}; ref = {:?}" , obj , operation . obj) , } } pub (super) fn set_last_access (& mut self , operation : Operation , path_id : usize , dpor_vv : & VersionVec ,) { match & mut self . entries [operation . obj . index] { Entry :: Arc (entry) => entry . set_last_access (operation . action . into () , path_id , dpor_vv) , Entry :: Atomic (entry) => { entry . set_last_access (operation . action . into () , path_id , dpor_vv) } Entry :: Mutex (entry) => entry . set_last_access (path_id , dpor_vv) , Entry :: Condvar (entry) => entry . set_last_access (path_id , dpor_vv) , Entry :: Notify (entry) => entry . set_last_access (path_id , dpor_vv) , Entry :: RwLock (entry) => entry . set_last_access (path_id , dpor_vv) , Entry :: Channel (entry) => { entry . set_last_access (operation . action . into () , path_id , dpor_vv) } _ => panic ! ("object is not branchable") , } } # [doc = " Panics if any leaks were detected"] pub (crate) fn check_for_leaks (& self) { for (index , entry) in self . entries . iter () . enumerate () { match entry { Entry :: Alloc (entry) => entry . check_for_leaks (index) , Entry :: Arc (entry) => entry . check_for_leaks (index) , Entry :: Channel (entry) => entry . check_for_leaks (index) , _ => { } } } } }
    };
}

impl_87!();