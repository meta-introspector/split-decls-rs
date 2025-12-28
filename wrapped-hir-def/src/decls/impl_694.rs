macro_rules! deps {
    () => {
        DefDatabase!();
        MacroId!();
    };
}

macro_rules! impl_694 {
    () => {
        deps!();
        impl MacroId { pub fn is_attribute (self , db : & dyn DefDatabase) -> bool { matches ! (self , MacroId :: ProcMacroId (it) if it . lookup (db) . kind == ProcMacroKind :: Attr) } }
    };
}

impl_694!()