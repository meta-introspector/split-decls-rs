macro_rules! SCx {
    () => {
        # [doc = " `TyCtxt` (and related cache datastructures) can't be move between threads."] # [doc = " However, there are various cx related functions which we want to be available to the builder and"] # [doc = " other compiler pieces. Here we define a small subset which has enough information and can be"] # [doc = " moved around more freely."] pub (crate) struct SCx < 'll > { pub llmod : & 'll llvm :: Module , pub llcx : & 'll llvm :: Context , pub isize_ty : & 'll Type , }
    };
}

SCx!()