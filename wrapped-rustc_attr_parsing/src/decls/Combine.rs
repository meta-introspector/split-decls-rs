macro_rules! deps {
    () => {
        AttributeParser!();
        CombineAttributeParser!();
        Stage!();
    };
}

macro_rules! Combine {
    () => {
        deps!();
        # [doc = " Use in combination with [`CombineAttributeParser`]."] # [doc = " `Combine<T: CombineAttributeParser>` implements [`AttributeParser`]."] pub (crate) struct Combine < T : CombineAttributeParser < S > , S : Stage > { phantom : PhantomData < (S , T) > , # [doc = " A list of all items produced by parsing attributes so far. One attribute can produce any amount of items."] items : ThinVec < < T as CombineAttributeParser < S > > :: Item > , # [doc = " The full span of the first attribute that was encountered."] first_span : Option < Span > , }
    };
}

Combine!();