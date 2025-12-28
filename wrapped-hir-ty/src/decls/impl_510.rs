macro_rules! deps {
    () => {
        HirFormatter!();
        HirDisplayError!();
        HirDisplay!();
        FnSig!();
        PolyFnSig!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for PolyFnSig < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { let FnSig { inputs_and_output , c_variadic , safety , abi : _ } = self . skip_binder () ; if let Safety :: Unsafe = safety { write ! (f , "unsafe ") ? ; } write ! (f , "fn(") ? ; f . write_joined (inputs_and_output . inputs () , ", ") ? ; if c_variadic { if inputs_and_output . inputs () . is_empty () { write ! (f , "...") ? ; } else { write ! (f , ", ...") ? ; } } write ! (f , ")") ? ; let ret = inputs_and_output . output () ; if ! ret . is_unit () { write ! (f , " -> ") ? ; ret . hir_fmt (f) ? ; } Ok (()) } }
    };
}

impl_510!()