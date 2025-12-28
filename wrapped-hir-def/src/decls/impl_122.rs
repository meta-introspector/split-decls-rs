macro_rules! deps {
    () => {
        MacroId!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl MacroId { pub fn is_attribute (self , db : & dyn DefDatabase) -> bool { matches ! (self , MacroId :: ProcMacroId (it) if it . lookup (db) . kind == ProcMacroKind :: Attr) } }
    };
}

impl_122!()