macro_rules! deps {
    () => {
        Oid!();
        Error!();
        RevwalkWithHideCb!();
    };
}

macro_rules! impl_717 {
    () => {
        deps!();
        impl < 'repo , 'cb , C : FnMut (Oid) -> bool > Iterator for RevwalkWithHideCb < 'repo , 'cb , C > { type Item = Result < Oid , Error > ; fn next (& mut self) -> Option < Result < Oid , Error > > { let out = self . revwalk . next () ; crate :: panic :: check () ; out } }
    };
}

impl_717!();