macro_rules! get_doc_string_in_attr {
    () => {
        fn get_doc_string_in_attr (it : & ast :: Attr) -> Option < ast :: String > { match it . expr () { Some (ast :: Expr :: Literal (lit)) => match lit . kind () { ast :: LiteralKind :: String (it) => Some (it) , _ => None , } , None => { None } _ => None , } }
    };
}

get_doc_string_in_attr!()