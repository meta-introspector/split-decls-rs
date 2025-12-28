macro_rules! deps {
    () => {
        AttributeSpecification!();
        Attributes!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl FromIterator < AttributeSpecification > for Attributes { fn from_iter < I > (iter : I) -> Attributes where I : IntoIterator < Item = AttributeSpecification > , { let mut list = Attributes :: new () ; for item in iter { list . push (item) ; } list } }
    };
}

impl_352!();