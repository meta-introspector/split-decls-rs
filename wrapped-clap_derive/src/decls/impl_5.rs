macro_rules! deps {
    () => {
        AttrKind!();
        AttrValue!();
        Sp!();
        ClapAttr!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl ClapAttr { pub (crate) fn parse_all (all_attrs : & [Attribute]) -> Result < Vec < Self > , syn :: Error > { let mut parsed = Vec :: new () ; for attr in all_attrs { let kind = if attr . path () . is_ident ("clap") { Sp :: new (AttrKind :: Clap , attr . path () . span ()) } else if attr . path () . is_ident ("structopt") { Sp :: new (AttrKind :: StructOpt , attr . path () . span ()) } else if attr . path () . is_ident ("command") { Sp :: new (AttrKind :: Command , attr . path () . span ()) } else if attr . path () . is_ident ("group") { Sp :: new (AttrKind :: Group , attr . path () . span ()) } else if attr . path () . is_ident ("arg") { Sp :: new (AttrKind :: Arg , attr . path () . span ()) } else if attr . path () . is_ident ("value") { Sp :: new (AttrKind :: Value , attr . path () . span ()) } else { continue ; } ; for mut attr in attr . parse_args_with (Punctuated :: < ClapAttr , Token ! [,] > :: parse_terminated) ? { attr . kind = kind ; parsed . push (attr) ; } } Ok (parsed) } pub (crate) fn value_or_abort (& self) -> Result < & AttrValue , syn :: Error > { self . value . as_ref () . ok_or_else (| | format_err ! (self . name , "attribute `{}` requires a value" , self . name)) } pub (crate) fn lit_str_or_abort (& self) -> Result < & LitStr , syn :: Error > { let value = self . value_or_abort () ? ; match value { AttrValue :: LitStr (tokens) => Ok (tokens) , AttrValue :: Expr (_) | AttrValue :: Call (_) => { abort ! (self . name , "attribute `{}` can only accept string literals" , self . name) } } } }
    };
}

impl_5!()