macro_rules! deps {
    () => {
        Attr!();
    };
}

macro_rules! inner_attributes {
    () => {
        deps!();
        fn inner_attributes (syntax : & SyntaxNode ,) -> Option < impl Iterator < Item = Either < ast :: Attr , ast :: Comment > > > { let node = match_ast ! { match syntax { ast :: SourceFile (_) => syntax . clone () , ast :: ExternBlock (it) => it . extern_item_list () ?. syntax () . clone () , ast :: Fn (it) => it . body () ?. stmt_list () ?. syntax () . clone () , ast :: Impl (it) => it . assoc_item_list () ?. syntax () . clone () , ast :: Module (it) => it . item_list () ?. syntax () . clone () , ast :: BlockExpr (it) => { if ! it . may_carry_attributes () { return None } syntax . clone () } , _ => return None , } } ; let attrs = ast :: AttrDocCommentIter :: from_syntax_node (& node) . filter (| el | match el { Either :: Left (attr) => attr . kind () . is_inner () , Either :: Right (comment) => comment . is_inner () , }) ; Some (attrs) }
    };
}

inner_attributes!()