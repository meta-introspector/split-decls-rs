macro_rules! deps {
    () => {
        Methods!();
        MockItemStruct!();
        MockTrait!();
        MockableStruct!();
        Builder!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl From < MockableStruct > for MockItemStruct { fn from (mockable : MockableStruct) -> MockItemStruct { let auto_debug = mockable . derives_debug () ; let modname = gen_mod_ident (& mockable . name , None) ; let generics = mockable . generics . clone () ; let struct_name = & mockable . name ; let vis = mockable . vis ; let has_new = mockable . methods . iter () . any (| meth | meth . sig . ident == "new") || mockable . impls . iter () . any (| impl_ | impl_ . items . iter () . any (| ii | if let ImplItem :: Fn (iif) = ii { iif . sig . ident == "new" } else { false })) ; let methods = Methods (mockable . methods . into_iter () . map (| meth | mock_function :: Builder :: new (& meth . sig , & meth . vis) . attrs (& meth . attrs) . struct_ (struct_name) . struct_generics (& generics) . levels (2) . call_levels (0) . build ()) . collect :: < Vec < _ > > ()) ; let structname = & mockable . name ; let traits = mockable . impls . into_iter () . map (| i | MockTrait :: new (structname , & generics , i , & vis)) . collect () ; MockItemStruct { attrs : mockable . attrs , auto_debug , consts : mockable . consts , generics , has_new , methods , modname , name : mockable . name , traits , vis } } }
    };
}

impl_69!()