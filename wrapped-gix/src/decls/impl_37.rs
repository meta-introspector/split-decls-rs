macro_rules! deps {
    () => {
        AttributeStack!();
        Note!();
        Repository!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'repo > AttributeStack < 'repo > { # [doc = " Create a new instance from a `repo` and the underlying pre-configured `stack`."] # [doc = ""] # [doc = " Note that this type is typically created by [`Repository::attributes()`] or [`Repository::attributes_only()`]."] pub fn new (stack : gix_worktree :: Stack , repo : & 'repo Repository) -> Self { AttributeStack { repo , inner : stack } } # [doc = " Detach the repository and return the underlying plumbing datatype."] pub fn detach (self) -> gix_worktree :: Stack { self . inner } }
    };
}

impl_37!()