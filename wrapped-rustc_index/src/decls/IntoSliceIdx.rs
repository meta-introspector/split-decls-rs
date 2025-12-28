macro_rules! IntoSliceIdx {
    () => {
        # [doc = " Helper trait for indexing operations with a custom index type."] pub trait IntoSliceIdx < I , T : ? Sized > { type Output : SliceIndex < T > ; fn into_slice_idx (self) -> Self :: Output ; }
    };
}

IntoSliceIdx!();