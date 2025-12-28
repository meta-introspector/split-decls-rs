macro_rules! extract_field_attributes {
    () => {
        # [doc = " Extract attributes from field, and return them"] # [doc = ""] # [doc = " Only current field attribute is `zerovec::varule(VarUleType)`"] pub fn extract_field_attributes (attrs : & mut Vec < Attribute >) -> Result < Option < Ident > > { let mut zerovec_attrs = extract_zerovec_attributes (attrs) ; let varule = extract_parenthetical_zerovec_attrs (& mut zerovec_attrs , "varule") ? ; if varule . len () > 1 { return Err (Error :: new (varule [1] . span () , "Found multiple #[zerovec::varule()] on one field" ,)) ; } if ! zerovec_attrs . is_empty () { return Err (Error :: new (zerovec_attrs [1] . span () , "Found unusable #[zerovec::] attrs on field, only #[zerovec::varule()] supported" ,)) ; } Ok (varule . first () . cloned ()) }
    };
}

extract_field_attributes!();