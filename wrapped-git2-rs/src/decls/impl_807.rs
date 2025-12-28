macro_rules! deps {
    () => {
        Binding!();
        Transaction!();
    };
}

macro_rules! impl_807 {
    () => {
        deps!();
        impl < 'repo > Binding for Transaction < 'repo > { type Raw = * mut raw :: git_transaction ; unsafe fn from_raw (ptr : * mut raw :: git_transaction) -> Transaction < 'repo > { Transaction { raw : ptr , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_transaction { self . raw } }
    };
}

impl_807!()