macro_rules! deps {
    () => {
        VecView!();
        Vec!();
        OwnedStorage!();
        ViewStorage!();
        SealedStorage!();
    };
}

macro_rules! Storage {
    () => {
        deps!();
        # [doc = " Trait defining how data for a container is stored."] # [doc = ""] # [doc = " There's two implementations available:"] # [doc = ""] # [doc = " - [`OwnedStorage`]: stores the data in an array `[T; N]` whose size is known at compile time."] # [doc = " - [`ViewStorage`]: stores the data in an unsized `[T]`."] # [doc = ""] # [doc = " This allows containers to be generic over either sized or unsized storage. For example,"] # [doc = " the [`vec`](crate::vec) module contains a [`VecInner`](crate::vec::VecInner) struct"] # [doc = " that's generic on [`Storage`], and two type aliases for convenience:"] # [doc = ""] # [doc = " - [`Vec<T, N>`](crate::vec::Vec) = `VecInner<T, OwnedStorage<N>>`"] # [doc = " - [`VecView<T>`](crate::vec::VecView) = `VecInner<T, ViewStorage>`"] # [doc = ""] # [doc = " `Vec` can be unsized into `VecView`, either by unsizing coercions such as `&mut Vec -> &mut"] # [doc = " VecView` or `Box<Vec> -> Box<VecView>`, or explicitly with"] # [doc = " [`.as_view()`](crate::vec::Vec::as_view) or [`.as_mut_view()`](crate::vec::Vec::as_mut_view)."] # [doc = ""] # [doc = " This trait is sealed, so you cannot implement it for your own types. You can only use"] # [doc = " the implementations provided by this crate."] # [allow (private_bounds)] pub trait Storage : SealedStorage { }
    };
}

Storage!();