macro_rules! deps {
    () => {
        Declaration!();
        ReferenceVisitor!();
        SymbolMap!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'a > ReferenceVisitor < 'a > { pub fn new (symbol_map : & 'a mut SymbolMap , declarations : & 'a mut Vec < Declaration > , crate_name : String , module_path : String , verbose : u8 ,) -> Self { ReferenceVisitor { symbol_map , declarations , crate_name , module_path , verbose , } } }
    };
}

impl_131!()