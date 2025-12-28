macro_rules! deps {
    () => {
        Bytes!();
        CrelIteratorState!();
        CrelIteratorHeader!();
    };
}

macro_rules! CrelIterator {
    () => {
        deps!();
        # [doc = " Compact relocation iterator."] # [derive (Debug , Clone)] pub struct CrelIterator < 'data > { # [doc = " Input stream reader."] data : Bytes < 'data > , # [doc = " Parsed header information."] header : CrelIteratorHeader , # [doc = " State of the iterator."] state : CrelIteratorState , }
    };
}

CrelIterator!()