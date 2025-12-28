macro_rules! deps {
    () => {
        AttributeTag!();
        Bytes!();
    };
}

macro_rules! AttributesSubsubsection {
    () => {
        deps!();
        # [doc = " A sub-subsection in an attributes section."] # [derive (Debug , Clone)] pub struct AttributesSubsubsection < 'data > { # [doc = " The sub-subsection tag."] pub tag : AttributeTag , # [doc = " The data containing the attributes."] pub data : Bytes < 'data > , }
    };
}

AttributesSubsubsection!();