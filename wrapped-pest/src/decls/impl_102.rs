macro_rules! deps {
    () => {
        BorrowedOrArc!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        # [doc = " A holder for a literal string, for use in `push_literal`. This is typically a `&'static str`, but is an owned"] # [doc = " `Rc<String>` for the pest vm."] impl < 'i > BorrowedOrArc < 'i > { fn as_str < 'a : 'i > (& 'a self) -> & 'a str { match self { BorrowedOrArc :: Borrowed (s) => s , BorrowedOrArc :: Owned (s) => s . deref () , } } }
    };
}

impl_102!()