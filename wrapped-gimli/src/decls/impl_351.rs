macro_rules! deps {
    () => {
        AttributeSpecification!();
        Attributes!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl Deref for Attributes { type Target = [AttributeSpecification] ; fn deref (& self) -> & [AttributeSpecification] { match self { Attributes :: Inline { buf , len } => & buf [.. * len] , Attributes :: Heap (list) => list , } } }
    };
}

impl_351!();