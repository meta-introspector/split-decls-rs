macro_rules! deps {
    () => {
        Error!();
        ReferenceNames!();
    };
}

macro_rules! impl_615 {
    () => {
        deps!();
        impl < 'repo , 'references > Iterator for ReferenceNames < 'repo , 'references > { type Item = Result < & 'references str , Error > ; fn next (& mut self) -> Option < Result < & 'references str , Error > > { let mut out = ptr :: null () ; unsafe { try_call_iter ! (raw :: git_reference_next_name (& mut out , self . inner . raw)) ; let bytes = crate :: opt_bytes (self , out) . unwrap () ; let s = str :: from_utf8 (bytes) . unwrap () ; Some (Ok (mem :: transmute :: < & str , & 'references str > (s))) } } }
    };
}

impl_615!()