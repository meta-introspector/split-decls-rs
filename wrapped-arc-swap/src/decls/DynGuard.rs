macro_rules! DynGuard {
    () => {
        # [doc (hidden)] pub struct DynGuard < T : ? Sized > (Box < dyn Deref < Target = T > >) ;
    };
}

DynGuard!();