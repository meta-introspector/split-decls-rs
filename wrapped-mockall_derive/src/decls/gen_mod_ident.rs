macro_rules! gen_mod_ident {
    () => {
        # [doc = " Generate an identifier for the mock struct's private module: eg \"Foo\" =>"] # [doc = " \"__mock_Foo\""] fn gen_mod_ident (struct_ : & Ident , trait_ : Option < & Ident >) -> Ident { if let Some (t) = trait_ { format_ident ! ("__mock_{struct_}_{}" , t) } else { format_ident ! ("__mock_{struct_}") } }
    };
}

gen_mod_ident!()