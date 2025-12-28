macro_rules! deps {
    () => {
        Enum!();
        Adt!();
        HasSource!();
        Struct!();
        Union!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl HasSource for Adt { type Ast = ast :: Adt ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { match self { Adt :: Struct (s) => Some (s . source (db) ? . map (ast :: Adt :: Struct)) , Adt :: Union (u) => Some (u . source (db) ? . map (ast :: Adt :: Union)) , Adt :: Enum (e) => Some (e . source (db) ? . map (ast :: Adt :: Enum)) , } } }
    };
}

impl_49!();