macro_rules! is_attribute_ident {
    () => {
        # [macro_export] macro_rules ! is_attribute_ident { ($ attr : expr , $ ident_str : expr) => { $ attr . path () . is_ident ($ ident_str) } ; }
    };
}

is_attribute_ident!()