macro_rules! deps {
    () => {
        Binding!();
        Refspec!();
    };
}

macro_rules! impl_634 {
    () => {
        deps!();
        impl < 'remote > Binding for Refspec < 'remote > { type Raw = * const raw :: git_refspec ; unsafe fn from_raw (raw : * const raw :: git_refspec) -> Refspec < 'remote > { Refspec { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_refspec { self . raw } }
    };
}

impl_634!();