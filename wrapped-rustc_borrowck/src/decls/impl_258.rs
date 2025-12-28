macro_rules! impl_258 {
    () => {
        impl Atom for BorrowIndex { fn index (self) -> usize { self . as_usize () } }
    };
}

impl_258!();