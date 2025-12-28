macro_rules! deps {
    () => {
        ReadRef!();
        CoffHeader!();
        CoffSymbolIterator!();
        CoffCommon!();
        SymbolIndex!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < 'data , 'file , R , Coff > CoffSymbolIterator < 'data , 'file , R , Coff > where R : ReadRef < 'data > , Coff : CoffHeader , { pub (crate) fn new (file : & 'file CoffCommon < 'data , R , Coff >) -> Self { Self { file , index : SymbolIndex (0) , } } pub (crate) fn empty (file : & 'file CoffCommon < 'data , R , Coff >) -> Self { Self { file , index : SymbolIndex (file . symbols . len ()) , } } }
    };
}

impl_238!()