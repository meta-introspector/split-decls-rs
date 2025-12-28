macro_rules! deps {
    () => {
        Access!();
        State!();
        VersionVec!();
        Arc!();
        RefModify!();
        Action!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl State { pub (super) fn check_for_leaks (& self , index : usize) { if self . ref_cnt != 0 { if self . allocated . is_captured () { panic ! ("Arc leaked.\n  Allocated: {}\n      Index: {}" , self . allocated , index) ; } else { panic ! ("Arc leaked.\n  Index: {}" , index) ; } } } pub (super) fn last_dependent_access (& self , action : Action) -> Option < & Access > { match action { Action :: RefInc => self . last_ref_inspect . as_ref () , Action :: RefDec => self . last_ref_dec . as_ref () , Action :: Inspect => match self . last_ref_modification { Some (RefModify :: RefInc) => self . last_ref_inc . as_ref () , Some (RefModify :: RefDec) => self . last_ref_dec . as_ref () , None => None , } , } } pub (super) fn set_last_access (& mut self , action : Action , path_id : usize , version : & VersionVec) { match action { Action :: RefInc => { self . last_ref_modification = Some (RefModify :: RefInc) ; Access :: set_or_create (& mut self . last_ref_inc , path_id , version) } Action :: RefDec => { self . last_ref_modification = Some (RefModify :: RefDec) ; Access :: set_or_create (& mut self . last_ref_dec , path_id , version) } Action :: Inspect => Access :: set_or_create (& mut self . last_ref_inspect , path_id , version) , } } }
    };
}

impl_28!();