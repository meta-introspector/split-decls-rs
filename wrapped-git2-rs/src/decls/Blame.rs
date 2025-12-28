macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Blame {
    () => {
        deps!();
        # [doc = " Opaque structure to hold blame results."] pub struct Blame < 'repo > { raw : * mut raw :: git_blame , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Blame!()