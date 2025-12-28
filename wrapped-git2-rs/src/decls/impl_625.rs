macro_rules! deps {
    () => {
        Binding!();
        ReflogEntry!();
    };
}

macro_rules! impl_625 {
    () => {
        deps!();
        impl < 'reflog > Binding for ReflogEntry < 'reflog > { type Raw = * const raw :: git_reflog_entry ; unsafe fn from_raw (raw : * const raw :: git_reflog_entry) -> ReflogEntry < 'reflog > { ReflogEntry { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_reflog_entry { self . raw } }
    };
}

impl_625!()