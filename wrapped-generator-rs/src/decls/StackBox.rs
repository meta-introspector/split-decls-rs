macro_rules! StackBox {
    () => {
        # [doc = " A pointer type for stack allocation."] pub struct StackBox < T > { ptr : ptr :: NonNull < T > , }
    };
}

StackBox!();