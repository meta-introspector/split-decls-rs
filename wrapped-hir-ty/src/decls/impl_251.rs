macro_rules! deps {
    () => {
        AutoBorrowMutability!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl From < AutoBorrowMutability > for Mutability { fn from (m : AutoBorrowMutability) -> Self { match m { AutoBorrowMutability :: Mut { .. } => Mutability :: Mut , AutoBorrowMutability :: Not => Mutability :: Not , } } }
    };
}

impl_251!();