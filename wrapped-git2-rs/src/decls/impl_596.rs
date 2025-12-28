macro_rules! deps {
    () => {
        Binding!();
        RebaseOperation!();
    };
}

macro_rules! impl_596 {
    () => {
        deps!();
        impl < 'rebase > Binding for RebaseOperation < 'rebase > { type Raw = * const raw :: git_rebase_operation ; unsafe fn from_raw (raw : * const raw :: git_rebase_operation) -> RebaseOperation < 'rebase > { RebaseOperation { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_rebase_operation { self . raw } }
    };
}

impl_596!();