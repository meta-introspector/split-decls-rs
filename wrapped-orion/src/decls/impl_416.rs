macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl Zeroize for FieldElement { fn zeroize (& mut self) { self . 0 . zeroize () ; } }
    };
}

impl_416!()