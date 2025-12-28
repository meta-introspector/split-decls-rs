macro_rules! deps {
    () => {
        Bytes!();
        AttributesSubsubsection!();
    };
}

macro_rules! AttributeIndexIterator {
    () => {
        deps!();
        # [doc = " An iterator over the indices in an [`AttributesSubsubsection`]."] # [derive (Debug , Clone)] pub struct AttributeIndexIterator < 'data > { data : Bytes < 'data > , }
    };
}

AttributeIndexIterator!()