macro_rules! deps {
    () => {
        GeneratorResult!();
    };
}

macro_rules! get_rustdoc {
    () => {
        deps!();
        pub fn get_rustdoc (attrs : & [Attribute]) -> GeneratorResult < Option < TokenStream > > { let mut full_docs : Vec < TokenStream > = vec ! [] ; let mut combined_docs_literal = String :: new () ; for attr in attrs { if let Meta :: NameValue (nv) = & attr . meta { if nv . path . is_ident ("doc") { match & nv . value { Expr :: Lit (ExprLit { lit : Lit :: Str (doc) , .. }) => { let doc = doc . value () ; let doc_str = doc . trim () ; combined_docs_literal += "\n" ; combined_docs_literal += doc_str ; } Expr :: Macro (include_macro) => { if ! combined_docs_literal . is_empty () { combined_docs_literal += "\n" ; let lit = LitStr :: new (& combined_docs_literal , Span :: call_site ()) ; full_docs . push (quote ! (# lit)) ; combined_docs_literal . clear () ; } full_docs . push (quote ! (# include_macro)) ; } _ => () , } } } } if ! combined_docs_literal . is_empty () { let lit = LitStr :: new (& combined_docs_literal , Span :: call_site ()) ; full_docs . push (quote ! (# lit)) ; combined_docs_literal . clear () ; } Ok (if full_docs . is_empty () { None } else { Some (quote ! (:: core :: primitive :: str :: trim (:: std :: concat ! (# (# full_docs) ,*)))) }) }
    };
}

get_rustdoc!();