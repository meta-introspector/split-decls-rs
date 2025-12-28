macro_rules! deps {
    () => {
        Refdb!();
    };
}

macro_rules! References {
    () => {
        deps!();
        # [doc = " An iterator over the references in a repository."] pub struct References < 'repo > { raw : * mut raw :: git_reference_iterator , _marker : marker :: PhantomData < Refdb < 'repo > > , }
    };
}

References!()