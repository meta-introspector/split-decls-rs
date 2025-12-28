macro_rules! deps {
    () => {
        Oid!();
        Error!();
        Binding!();
        Revwalk!();
    };
}

macro_rules! impl_716 {
    () => {
        deps!();
        impl < 'repo > Iterator for Revwalk < 'repo > { type Item = Result < Oid , Error > ; fn next (& mut self) -> Option < Result < Oid , Error > > { let mut out : raw :: git_oid = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call_iter ! (raw :: git_revwalk_next (& mut out , self . raw ())) ; Some (Ok (Binding :: from_raw (& out as * const _))) } } }
    };
}

impl_716!()