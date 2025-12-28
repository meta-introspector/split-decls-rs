macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < & 'a [u8] > { if self . cur . is_null () { return None ; } unsafe { let ret = Some (CStr :: from_ptr ((* self . cur) . data) . to_bytes ()) ; self . cur = (* self . cur) . next ; ret } } }
    };
}

impl_108!()