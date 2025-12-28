macro_rules! sval_attr {
    () => {
        fn sval_attr < 'a > (ctxt : & 'a str , attr : & '_ Attribute ,) -> Option < impl IntoIterator < Item = (Path , Expr) > + 'a > { if ! attr . path () . is_ident ("sval") { return None ; } let mut results = Vec :: new () ; attr . parse_nested_meta (| meta | { let expr : Expr = match meta . value () { Ok (value) => value . parse () ? , Err (_) => syn :: parse_quote ! (true) , } ; let path = meta . path ; results . push ((path , expr)) ; Ok (()) }) . unwrap_or_else (| e | panic ! ("failed to parse attribute on {}: {}" , ctxt , e)) ; Some (results) }
    };
}

sval_attr!()