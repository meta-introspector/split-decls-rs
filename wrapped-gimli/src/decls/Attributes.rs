macro_rules! deps {
    () => {
        AttributeSpecification!();
        Abbreviation!();
    };
}

macro_rules! Attributes {
    () => {
        deps!();
        # [doc = " A list of attributes found in an `Abbreviation`"] # [derive (Clone)] pub (crate) enum Attributes { Inline { buf : [AttributeSpecification ; MAX_ATTRIBUTES_INLINE] , len : usize , } , Heap (Vec < AttributeSpecification >) , }
    };
}

Attributes!();