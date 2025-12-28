macro_rules! deps {
    () => {
        Endian!();
        FileHeader!();
        AttributesSection!();
        Bytes!();
    };
}

macro_rules! AttributesSubsectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the subsections in an [`AttributesSection`]."] # [derive (Debug , Clone)] pub struct AttributesSubsectionIterator < 'data , Elf : FileHeader > { endian : Elf :: Endian , data : Bytes < 'data > , }
    };
}

AttributesSubsectionIterator!()