macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        # [doc = " `Unique` pointers are `Sync` if `T` is `Sync` because the data they"] # [doc = " reference is unaliased. Note that this aliasing invariant is"] # [doc = " unenforced by the type system; the abstraction using the"] # [doc = " `Unique` must enforce it."] unsafe impl < T : Sync + ? Sized > Sync for Unique < T > { }
    };
}

impl_206!()