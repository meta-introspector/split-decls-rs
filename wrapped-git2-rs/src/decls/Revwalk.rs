macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! Revwalk {
    () => {
        deps!();
        # [doc = " A revwalk allows traversal of the commit graph defined by including one or"] # [doc = " more leaves and excluding one or more roots."] pub struct Revwalk < 'repo > { raw : * mut raw :: git_revwalk , _marker : marker :: PhantomData < & 'repo Repository > , }
    };
}

Revwalk!();