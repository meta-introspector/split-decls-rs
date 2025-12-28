macro_rules! deps {
    () => {
        HeaderSlice!();
        Arc!();
        ArcInner!();
    };
}

macro_rules! ThinArc {
    () => {
        deps!();
        # [doc = " A \"thin\" `Arc` containing dynamically sized data"] # [doc = ""] # [doc = " This is functionally equivalent to `Arc<(H, [T])>`"] # [doc = ""] # [doc = " When you create an `Arc` containing a dynamically sized type"] # [doc = " like `HeaderSlice<H, [T]>`, the `Arc` is represented on the stack"] # [doc = " as a \"fat pointer\", where the length of the slice is stored"] # [doc = " alongside the `Arc`'s pointer. In some situations you may wish to"] # [doc = " have a thin pointer instead, perhaps for FFI compatibility"] # [doc = " or space efficiency."] # [doc = ""] # [doc = " Note that we use `[T; 0]` in order to have the right alignment for `T`."] # [doc = ""] # [doc = " `ThinArc` solves this by storing the length in the allocation itself,"] # [doc = " via `HeaderSlice`."] # [repr (transparent)] pub (crate) struct ThinArc < H , T > { ptr : ptr :: NonNull < ArcInner < HeaderSlice < H , [T ; 0] > > > , phantom : PhantomData < (H , T) > , }
    };
}

ThinArc!();