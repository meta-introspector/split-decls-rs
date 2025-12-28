macro_rules! deps {
    () => {
        LifetimeParam!();
        HasSource!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl HasSource for LifetimeParam { type Ast = ast :: LifetimeParam ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let child_source = self . id . parent . child_source (db) ; child_source . map (| it | it . get (self . id . local_id) . cloned ()) . transpose () } }
    };
}

impl_63!();