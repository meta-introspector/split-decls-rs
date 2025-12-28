macro_rules! deps {
    () => {
        SymbolIndex!();
        CoffHeader!();
        CoffComdatIterator!();
        ReadRef!();
        CoffFile!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > CoffComdatIterator < 'data , 'file , R , Coff > { pub (crate) fn new (file : & 'file CoffFile < 'data , R , Coff >) -> Self { CoffComdatIterator { file , index : SymbolIndex (0) , } } }
    };
}

impl_259!();