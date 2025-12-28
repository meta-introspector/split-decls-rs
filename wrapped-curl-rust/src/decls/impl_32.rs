macro_rules! deps {
    () => {
        Protocols!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a > Iterator for Protocols < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { unsafe { if (* self . cur) . is_null () { return None ; } let ret = crate :: opt_str (* self . cur) . unwrap () ; self . cur = self . cur . offset (1) ; Some (ret) } } }
    };
}

impl_32!();