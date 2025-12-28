macro_rules! is_pinned {
    () => {
        # [doc = " Returns `true` if the current thread is pinned."] # [inline] pub fn is_pinned () -> bool { with_handle (| handle | handle . is_pinned ()) }
    };
}

is_pinned!()