macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Drop for Owned < T > { fn drop (& mut self) { let (raw , _) = decompose_tag :: < T > (self . data) ; unsafe { T :: drop (raw) ; } } }
    };
}

impl_38!();