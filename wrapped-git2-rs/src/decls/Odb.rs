macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! Odb {
    () => {
        deps!();
        # [doc = " A structure to represent a git object database"] pub struct Odb < 'repo > { raw : * mut raw :: git_odb , _marker : marker :: PhantomData < Object < 'repo > > , }
    };
}

Odb!();