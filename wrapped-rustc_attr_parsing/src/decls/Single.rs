macro_rules! deps {
    () => {
        SingleAttributeParser!();
        AttributeParser!();
        Stage!();
    };
}

macro_rules! Single {
    () => {
        deps!();
        # [doc = " Use in combination with [`SingleAttributeParser`]."] # [doc = " `Single<T: SingleAttributeParser>` implements [`AttributeParser`]."] pub (crate) struct Single < T : SingleAttributeParser < S > , S : Stage > (PhantomData < (S , T) > , Option < (AttributeKind , Span) > ,) ;
    };
}

Single!()