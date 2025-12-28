macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! usize_const {
    () => {
        deps!();
        # [doc = " Interns a possibly-unknown target usize"] pub fn usize_const < 'db > (db : & 'db dyn HirDatabase , value : Option < u128 > , krate : Crate) -> Const < 'db > { intern_const_ref (db , & value . map_or (LiteralConstRef :: Unknown , LiteralConstRef :: UInt) , Ty :: new_uint (DbInterner :: new_with (db , Some (krate) , None) , rustc_type_ir :: UintTy :: Usize) , krate ,) }
    };
}

usize_const!();