macro_rules! deps {
    () => {
        References!();
        Binding!();
        Error!();
        Reference!();
    };
}

macro_rules! impl_613 {
    () => {
        deps!();
        impl < 'repo > Iterator for References < 'repo > { type Item = Result < Reference < 'repo > , Error > ; fn next (& mut self) -> Option < Result < Reference < 'repo > , Error > > { let mut out = ptr :: null_mut () ; unsafe { try_call_iter ! (raw :: git_reference_next (& mut out , self . raw)) ; Some (Ok (Binding :: from_raw (out))) } } }
    };
}

impl_613!()