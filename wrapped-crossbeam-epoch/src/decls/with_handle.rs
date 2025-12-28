macro_rules! deps {
    () => {
        LocalHandle!();
    };
}

macro_rules! with_handle {
    () => {
        deps!();
        # [inline] fn with_handle < F , R > (mut f : F) -> R where F : FnMut (& LocalHandle) -> R , { HANDLE . try_with (| h | f (h)) . unwrap_or_else (| _ | f (& collector () . register ())) }
    };
}

with_handle!();