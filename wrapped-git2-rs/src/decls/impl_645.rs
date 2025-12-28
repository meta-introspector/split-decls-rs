macro_rules! deps {
    () => {
        Remote!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl < 'repo > Clone for Remote < 'repo > { fn clone (& self) -> Remote < 'repo > { let mut ret = ptr :: null_mut () ; let rc = unsafe { call ! (raw :: git_remote_dup (& mut ret , self . raw)) } ; assert_eq ! (rc , 0) ; Remote { raw : ret , _marker : marker :: PhantomData , } } }
    };
}

impl_645!();