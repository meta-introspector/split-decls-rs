macro_rules! deps {
    () => {
        HasSource!();
        Field!();
        FieldSource!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl HasSource for Field { type Ast = FieldSource ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let var = VariantId :: from (self . parent) ; let src = var . child_source (db) ; let field_source = src . map (| it | match it [self . id] . clone () { Either :: Left (it) => FieldSource :: Pos (it) , Either :: Right (it) => FieldSource :: Named (it) , }) ; Some (field_source) } }
    };
}

impl_48!()