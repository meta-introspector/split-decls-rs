macro_rules! deps {
    () => {
        SimpleCx!();
        SCx!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < 'll > SimpleCx < 'll > { pub (crate) fn new (llmod : & 'll llvm :: Module , llcx : & 'll llvm :: Context , pointer_size : Size ,) -> Self { let isize_ty = llvm :: Type :: ix_llcx (llcx , pointer_size . bits ()) ; Self (SCx { llmod , llcx , isize_ty } , PhantomData) } }
    };
}

impl_220!()