macro_rules! gen_value_variants {
    () => {
        fn gen_value_variants (lits : & [(TokenStream , Ident)]) -> TokenStream { let lit = lits . iter () . map (| l | & l . 1) . collect :: < Vec < _ > > () ; quote ! { fn value_variants <'a > () -> &'a [Self] { & [# (Self ::# lit) ,*] } } }
    };
}

gen_value_variants!()