macro_rules! deps {
    () => {
        FileHeader!();
        Endian!();
        AttributesSubsection!();
        Bytes!();
    };
}

macro_rules! AttributesSubsubsectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sub-subsections in an [`AttributesSubsection`]."] # [derive (Debug , Clone)] pub struct AttributesSubsubsectionIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
    };
}

AttributesSubsubsectionIterator!()