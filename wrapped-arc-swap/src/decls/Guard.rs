macro_rules! Guard {
    () => {
        # [doc = " A temporary storage of the pointer."] # [doc = ""] # [doc = " This guard object is returned from most loading methods (with the notable exception of"] # [doc = " [`load_full`](struct.ArcSwapAny.html#method.load_full)). It dereferences to the smart pointer"] # [doc = " loaded, so most operations are to be done using that."] pub struct Guard < T : RefCnt , S : Strategy < T > = DefaultStrategy > { inner : S :: Protected , }
    };
}

Guard!()