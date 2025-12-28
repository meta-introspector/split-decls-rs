macro_rules! deps {
    () => {
        ElementIterator!();
        Element!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < N , E , I : ? Sized > ElementIterator < N , E > for I where I : Iterator < Item = Element < N , E > > { }
    };
}

impl_248!();