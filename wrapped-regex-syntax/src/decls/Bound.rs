macro_rules! Bound {
    () => {
        pub trait Bound : Copy + Clone + Debug + Eq + PartialEq + PartialOrd + Ord { fn min_value () -> Self ; fn max_value () -> Self ; fn as_u32 (self) -> u32 ; fn increment (self) -> Self ; fn decrement (self) -> Self ; }
    };
}

Bound!();