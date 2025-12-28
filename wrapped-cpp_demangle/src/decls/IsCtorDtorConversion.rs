macro_rules! deps {
    () => {
        SubstitutionTable!();
    };
}

macro_rules! IsCtorDtorConversion {
    () => {
        deps!();
        # [doc = " Determine whether this AST node is a constructor, destructor, or conversion"] # [doc = " function."] pub (crate) trait IsCtorDtorConversion { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool ; }
    };
}

IsCtorDtorConversion!()