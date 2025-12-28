macro_rules! deps {
    () => {
        TypeOrConstParam!();
        Trait!();
        HasSource!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl HasSource for TypeOrConstParam { type Ast = Either < ast :: TypeOrConstParam , ast :: Trait > ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let child_source = self . id . parent . child_source (db) ; child_source . map (| it | it . get (self . id . local_id) . cloned ()) . transpose () } }
    };
}

impl_62!()