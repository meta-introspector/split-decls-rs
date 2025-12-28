macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        # [doc = " `Unique` pointers are `Send` if `T` is `Send` because the data they"] # [doc = " reference is unaliased. Note that this aliasing invariant is"] # [doc = " unenforced by the type system; the abstraction using the"] # [doc = " `Unique` must enforce it."] unsafe impl < T : Send + ? Sized > Send for Unique < T > { }
    };
}

impl_205!();