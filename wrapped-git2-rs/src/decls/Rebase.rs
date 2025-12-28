macro_rules! Rebase {
    () => {
        # [doc = " Representation of a rebase"] pub struct Rebase < 'repo > { raw : * mut raw :: git_rebase , _marker : marker :: PhantomData < & 'repo raw :: git_rebase > , }
    };
}

Rebase!();