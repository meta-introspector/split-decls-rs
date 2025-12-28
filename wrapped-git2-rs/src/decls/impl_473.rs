macro_rules! deps {
    () => {
        Notes!();
        Binding!();
        Error!();
        Oid!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < 'repo > Iterator for Notes < 'repo > { type Item = Result < (Oid , Oid) , Error > ; fn next (& mut self) -> Option < Result < (Oid , Oid) , Error > > { let mut note_id = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; let mut annotated_id = note_id ; unsafe { try_call_iter ! (raw :: git_note_next (& mut note_id , & mut annotated_id , self . raw)) ; Some (Ok ((Binding :: from_raw (& note_id as * const _) , Binding :: from_raw (& annotated_id as * const _) ,))) } } }
    };
}

impl_473!();