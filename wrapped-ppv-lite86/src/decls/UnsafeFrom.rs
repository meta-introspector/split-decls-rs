macro_rules! UnsafeFrom {
    () => {
        # [allow (clippy :: missing_safety_doc)] pub trait UnsafeFrom < T > { unsafe fn unsafe_from (t : T) -> Self ; }
    };
}

UnsafeFrom!();