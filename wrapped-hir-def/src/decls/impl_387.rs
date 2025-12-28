macro_rules! deps {
    () => {
        AstIdLoc!();
        HasSource!();
        DefDatabase!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl < T > HasSource for T where T : AstIdLoc , { type Value = T :: Ast ; fn ast_ptr (& self , db : & dyn DefDatabase) -> InFile < AstPtr < Self :: Value > > { let id = self . ast_id () ; let ast_id_map = db . ast_id_map (id . file_id) ; InFile :: new (id . file_id , ast_id_map . get (id . value)) } }
    };
}

impl_387!();