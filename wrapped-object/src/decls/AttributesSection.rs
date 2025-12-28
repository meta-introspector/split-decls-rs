macro_rules! deps {
    () => {
        AttributesSubsection!();
    };
}

macro_rules! AttributesSection {
    () => {
        deps!();
        # [doc = " The contents of an attributes section."] # [derive (Debug , Default , Clone)] pub struct AttributesSection < 'data > { # [doc = " The subsections."] pub subsections : Vec < AttributesSubsection < 'data > > , }
    };
}

AttributesSection!();