macro_rules! deps {
    () => {
        References!();
    };
}

macro_rules! Branches {
    () => {
        deps!();
        # [doc = " An iterator over the branches inside of a repository."] pub struct Branches < 'repo > { raw : * mut raw :: git_branch_iterator , _marker : marker :: PhantomData < References < 'repo > > , }
    };
}

Branches!()