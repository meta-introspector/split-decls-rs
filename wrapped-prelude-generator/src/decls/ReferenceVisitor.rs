macro_rules! deps {
    () => {
        SymbolMap!();
        Declaration!();
    };
}

macro_rules! ReferenceVisitor {
    () => {
        deps!();
        pub struct ReferenceVisitor < 'a > { pub symbol_map : & 'a mut SymbolMap , pub declarations : & 'a mut Vec < Declaration > , pub crate_name : String , pub module_path : String , pub verbose : u8 , }
    };
}

ReferenceVisitor!()