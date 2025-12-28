macro_rules! Swap64 {
    () => {
        # [doc = " Exchange neigboring ranges of bits of the specified size"] pub trait Swap64 { fn swap1 (self) -> Self ; fn swap2 (self) -> Self ; fn swap4 (self) -> Self ; fn swap8 (self) -> Self ; fn swap16 (self) -> Self ; fn swap32 (self) -> Self ; fn swap64 (self) -> Self ; }
    };
}

Swap64!()