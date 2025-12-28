macro_rules! deps {
    () => {
        HoverResult!();
        RangeInfo!();
        Analysis!();
        HoverConfig!();
    };
}

macro_rules! hover_ranged {
    () => {
        deps!();
        fn hover_ranged (sema : & Semantics < '_ , RootDatabase > , FileRange { file_id , range } : FileRange , file : SyntaxNode , config : & HoverConfig < '_ > , edition : Edition , display_target : DisplayTarget ,) -> Option < RangeInfo < HoverResult > > { let expr_or_pat = file . covering_element (range) . ancestors () . take_while (| it | ast :: MacroCall :: can_cast (it . kind ()) || ! ast :: Item :: can_cast (it . kind ())) . find_map (Either :: < ast :: Expr , ast :: Pat > :: cast) ? ; let res = match & expr_or_pat { Either :: Left (ast :: Expr :: TryExpr (try_expr)) => { render :: try_expr (sema , config , try_expr , edition , display_target) } Either :: Left (ast :: Expr :: PrefixExpr (prefix_expr)) if prefix_expr . op_kind () == Some (ast :: UnaryOp :: Deref) => { render :: deref_expr (sema , config , prefix_expr , edition , display_target) } Either :: Left (ast :: Expr :: Literal (literal)) => { if let Some (literal) = ast :: String :: cast (literal . token ()) && let Some ((analysis , fixture_analysis)) = Analysis :: from_ra_fixture (sema , literal . clone () , & literal , config . minicore) { let (virtual_file_id , virtual_range) = fixture_analysis . map_range_down (range) ? ; return analysis . hover (config , FileRange { file_id : virtual_file_id , range : virtual_range }) . ok () ? ? . upmap_from_ra_fixture (& fixture_analysis , virtual_file_id , file_id) . ok () ; } None } _ => None , } ; let res = res . or_else (| | render :: type_info_of (sema , config , & expr_or_pat , edition , display_target)) ; res . map (| it | { let range = match expr_or_pat { Either :: Left (it) => it . syntax () . text_range () , Either :: Right (it) => it . syntax () . text_range () , } ; RangeInfo :: new (range , it) }) }
    };
}

hover_ranged!()