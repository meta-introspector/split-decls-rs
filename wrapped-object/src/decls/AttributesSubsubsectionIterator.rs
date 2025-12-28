macro_rules! deps {
    () => {
        Bytes!();
        AttributesSubsection!();
        FileHeader!();
        Endian!();
    };
}

macro_rules! AttributesSubsubsectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sub-subsections in an [`AttributesSubsection`]."] # [derive (Debug , Clone)] pub struct AttributesSubsubsectionIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
    };
}

AttributesSubsubsectionIterator!();