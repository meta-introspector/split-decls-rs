macro_rules! value_of_attribute {
    () => {
        fn value_of_attribute (requested : & str , attr : & Attribute) -> Option < String > { let value = match & attr . meta { Meta :: NameValue (meta) if meta . path . is_ident (requested) => & meta . value , _ => return None , } ; let lit = match value { Expr :: Lit (expr) if expr . attrs . is_empty () => & expr . lit , _ => return None , } ; match lit { Lit :: Str (string) => Some (string . value ()) , _ => None , } }
    };
}

value_of_attribute!();