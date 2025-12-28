macro_rules! OutRef {
    () => {
        # [doc = " A borrowed type with the same memory layout as the type itself that can be used to construct ABI-compatible function signatures."] # [doc = ""] # [doc = " This is a mutable version of [Ref] meant to support out parameters."] # [repr (transparent)] pub struct OutRef < 'a , T : Type < T > > (* mut T :: Abi , core :: marker :: PhantomData < & 'a T >) ;
    };
}

OutRef!();