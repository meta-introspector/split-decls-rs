macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! BufferKind {
    () => {
        deps!();
        # [doc = " Trait for buffer kinds."] pub trait BufferKind : sealed :: Sealed { }
    };
}

BufferKind!();