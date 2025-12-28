macro_rules! deps {
    () => {
        AstNodeWrapper!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < T , Tag > From < AstNodeWrapper < Box < T > , Tag > > for AstNodeWrapper < T , Tag > { fn from (value : AstNodeWrapper < Box < T > , Tag >) -> Self { AstNodeWrapper { wrapped : * value . wrapped , tag : value . tag } } }
    };
}

impl_266!()