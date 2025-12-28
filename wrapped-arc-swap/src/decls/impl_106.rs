macro_rules! deps {
    () => {
        RefCnt!();
        HybridProtection!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < T : RefCnt > Drop for HybridProtection < T > { # [inline] fn drop (& mut self) { match self . debt . take () { None => () , Some (debt) => { let ptr = T :: as_ptr (& self . ptr) ; if debt . pay :: < T > (ptr) { return ; } } } unsafe { ManuallyDrop :: drop (& mut self . ptr) } ; } }
    };
}

impl_106!();