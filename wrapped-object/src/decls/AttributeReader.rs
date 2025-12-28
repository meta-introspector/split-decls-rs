macro_rules! deps {
    () => {
        AttributesSubsubsection!();
        Bytes!();
    };
}

macro_rules! AttributeReader {
    () => {
        deps!();
        # [doc = " A parser for the attributes in an [`AttributesSubsubsection`]."] # [doc = ""] # [doc = " The parser relies on the caller to know the format of the data for each attribute tag."] # [derive (Debug , Clone)] pub struct AttributeReader < 'data > { data : Bytes < 'data > , }
    };
}

AttributeReader!()