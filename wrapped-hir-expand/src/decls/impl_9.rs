macro_rules! deps {
    () => {
        Attr!();
        ExpandDatabase!();
        AttrInput!();
        ModPath!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Attr { # [doc = " #[path = \"string\"]"] pub fn string_value (& self) -> Option < & Symbol > { match self . input . as_deref () ? { AttrInput :: Literal (tt :: Literal { symbol : text , kind : tt :: LitKind :: Str | tt :: LitKind :: StrRaw (_) , .. }) => Some (text) , _ => None , } } # [doc = " #[path = \"string\"]"] pub fn string_value_with_span (& self) -> Option < (& Symbol , span :: Span) > { match self . input . as_deref () ? { AttrInput :: Literal (tt :: Literal { symbol : text , kind : tt :: LitKind :: Str | tt :: LitKind :: StrRaw (_) , span , suffix : _ , }) => Some ((text , * span)) , _ => None , } } pub fn string_value_unescape (& self) -> Option < Cow < '_ , str > > { match self . input . as_deref () ? { AttrInput :: Literal (tt :: Literal { symbol : text , kind : tt :: LitKind :: StrRaw (_) , .. }) => Some (Cow :: Borrowed (text . as_str ())) , AttrInput :: Literal (tt :: Literal { symbol : text , kind : tt :: LitKind :: Str , .. }) => { unescape (text . as_str ()) } _ => None , } } # [doc = " #[path(ident)]"] pub fn single_ident_value (& self) -> Option < & tt :: Ident > { match self . input . as_deref () ? { AttrInput :: TokenTree (tt) => match tt . token_trees () . flat_tokens () { [tt :: TokenTree :: Leaf (tt :: Leaf :: Ident (ident))] => Some (ident) , _ => None , } , _ => None , } } # [doc = " #[path TokenTree]"] pub fn token_tree_value (& self) -> Option < & TopSubtree > { match self . input . as_deref () ? { AttrInput :: TokenTree (tt) => Some (tt) , _ => None , } } # [doc = " Parses this attribute as a token tree consisting of comma separated paths."] pub fn parse_path_comma_token_tree < 'a > (& 'a self , db : & 'a dyn ExpandDatabase ,) -> Option < impl Iterator < Item = (ModPath , Span) > + 'a > { let args = self . token_tree_value () ? ; if args . top_subtree () . delimiter . kind != DelimiterKind :: Parenthesis { return None ; } let paths = args . token_trees () . split (| tt | matches ! (tt , tt :: TtElement :: Leaf (tt :: Leaf :: Punct (Punct { char : ',' , .. })))) . filter_map (move | tts | { let span = tts . flat_tokens () . first () ? . first_span () ; Some ((ModPath :: from_tt (db , tts) ? , span)) }) ; Some (paths) } pub fn cfg (& self) -> Option < CfgExpr > { if * self . path . as_ident () ? == sym :: cfg { self . token_tree_value () . map (CfgExpr :: parse) } else { None } } }
    };
}

impl_9!()