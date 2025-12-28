macro_rules! deps {
    () => {
        AttributeSpecification!();
        Attributes!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl From < Vec < AttributeSpecification > > for Attributes { fn from (list : Vec < AttributeSpecification >) -> Attributes { Attributes :: Heap (list) } }
    };
}

impl_353!();