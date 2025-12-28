macro_rules! deps {
    () => {
        ProcMacro!();
        ExpandDatabase!();
        Attr!();
        AttrInput!();
        AttrId!();
        ModPath!();
        SpanMapRef!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Attr { fn from_src (db : & dyn ExpandDatabase , ast : ast :: Meta , span_map : SpanMapRef < '_ > , id : AttrId ,) -> Option < Attr > { let path = ast . path () ? ; let range = path . syntax () . text_range () ; let path = Interned :: new (ModPath :: from_src (db , path , & mut | range | { span_map . span_for_range (range) . ctx }) ?) ; let span = span_map . span_for_range (range) ; let input = if let Some (ast :: Expr :: Literal (lit)) = ast . expr () { let token = lit . token () ; Some (Box :: new (AttrInput :: Literal (token_to_literal (token . text () , span)))) } else if let Some (tt) = ast . token_tree () { let tree = syntax_node_to_token_tree (tt . syntax () , span_map , span , DocCommentDesugarMode :: ProcMacro ,) ; Some (Box :: new (AttrInput :: TokenTree (tree))) } else { None } ; Some (Attr { id , path , input , ctxt : span . ctx }) } fn from_tt (db : & dyn ExpandDatabase , mut tt : tt :: TokenTreesView < '_ > , id : AttrId ,) -> Option < Attr > { if matches ! (tt . flat_tokens () , [tt :: TokenTree :: Leaf (tt :: Leaf :: Ident (tt :: Ident { sym , .. })) , ..] if * sym == sym :: unsafe_) { match tt . iter () . nth (1) { Some (tt :: TtElement :: Subtree (_ , iter)) => tt = iter . remaining () , _ => return None , } } let first = tt . flat_tokens () . first () ? ; let ctxt = first . first_span () . ctx ; let (path , input) = { let mut iter = tt . iter () ; let start = iter . savepoint () ; let mut input = tt :: TokenTreesView :: new (& []) ; let mut path = iter . from_savepoint (start) ; let mut path_split_savepoint = iter . savepoint () ; while let Some (tt) = iter . next () { path = iter . from_savepoint (start) ; if ! matches ! (tt , tt :: TtElement :: Leaf (tt :: Leaf :: Punct (tt :: Punct { char : ':' | '$' , .. }) | tt :: Leaf :: Ident (_) ,)) { input = path_split_savepoint . remaining () ; break ; } path_split_savepoint = iter . savepoint () ; } (path , input) } ; let path = Interned :: new (ModPath :: from_tt (db , path) ?) ; let input = match (input . flat_tokens () . first () , input . try_into_subtree ()) { (_ , Some (tree)) => { Some (Box :: new (AttrInput :: TokenTree (tt :: TopSubtree :: from_subtree (tree)))) } (Some (tt :: TokenTree :: Leaf (tt :: Leaf :: Punct (tt :: Punct { char : '=' , .. }))) , _) => { match input . flat_tokens () . get (1) { Some (tt :: TokenTree :: Leaf (tt :: Leaf :: Literal (lit))) => { Some (Box :: new (AttrInput :: Literal (lit . clone ()))) } _ => None , } } _ => None , } ; Some (Attr { id , path , input , ctxt }) } pub fn path (& self) -> & ModPath { & self . path } pub fn expand_cfg_attr (self , db : & dyn ExpandDatabase , cfg_options : & CfgOptions ,) -> impl IntoIterator < Item = Self > { let is_cfg_attr = self . path . as_ident () . is_some_and (| name | * name == sym :: cfg_attr) ; if ! is_cfg_attr { return smallvec ! [self] ; } let subtree = match self . token_tree_value () { Some (it) => it , _ => return smallvec ! [self . clone ()] , } ; let (cfg , parts) = match parse_cfg_attr_input (subtree) { Some (it) => it , None => return smallvec ! [self . clone ()] , } ; let index = self . id ; let attrs = parts . filter_map (| attr | Attr :: from_tt (db , attr , index)) ; let cfg = TopSubtree :: from_token_trees (subtree . top_subtree () . delimiter , cfg) ; let cfg = CfgExpr :: parse (& cfg) ; if cfg_options . check (& cfg) == Some (false) { smallvec ! [] } else { cov_mark :: hit ! (cfg_attr_active) ; attrs . collect :: < SmallVec < _ , 1 > > () } } }
    };
}

impl_8!();