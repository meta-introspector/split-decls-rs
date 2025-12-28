macro_rules! deps {
    () => {
        AttributesSubsubsection!();
        ByteString!();
    };
}

macro_rules! AttributesSubsection {
    () => {
        deps!();
        # [doc = " A subsection of an attributes section."] # [derive (Debug , Clone)] pub struct AttributesSubsection < 'data > { # [doc = " The vendor namespace for these attributes."] pub vendor : ByteString < 'data > , # [doc = " The sub-subsections."] pub subsubsections : Vec < AttributesSubsubsection < 'data > > , }
    };
}

AttributesSubsection!();