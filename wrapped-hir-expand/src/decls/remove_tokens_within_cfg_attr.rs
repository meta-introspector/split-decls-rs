macro_rules! deps {
    () => {
        CfgExprStage!();
    };
}

macro_rules! remove_tokens_within_cfg_attr {
    () => {
        deps!();
        # [doc = " This function creates its own set of tokens to remove. To help prevent malformed syntax as input."] fn remove_tokens_within_cfg_attr (meta : Meta) -> Option < FxHashSet < SyntaxElement > > { let mut remove : FxHashSet < SyntaxElement > = FxHashSet :: default () ; debug ! ("Enabling attribute {}" , meta) ; let meta_path = meta . path () ? ; debug ! ("Removing {:?}" , meta_path . syntax ()) ; remove . insert (meta_path . syntax () . clone () . into ()) ; let meta_tt = meta . token_tree () ? ; debug ! ("meta_tt {}" , meta_tt) ; let mut stage = CfgExprStage :: StrippigCfgExpr ; for tt in meta_tt . token_trees_and_tokens () { debug ! ("Checking {:?}. Stage: {:?}" , tt , stage) ; match (stage , tt) { (CfgExprStage :: StrippigCfgExpr , syntax :: NodeOrToken :: Node (node)) => { remove . insert (node . syntax () . clone () . into ()) ; } (CfgExprStage :: StrippigCfgExpr , syntax :: NodeOrToken :: Token (token)) => { if token . kind () == T ! [,] { stage = CfgExprStage :: FoundComma ; } remove . insert (token . into ()) ; } (CfgExprStage :: FoundComma , syntax :: NodeOrToken :: Token (token)) if (token . kind () == T ! [,] || token . kind () == T ! [')']) => { stage = CfgExprStage :: EverythingElse ; remove . insert (token . into ()) ; } (CfgExprStage :: EverythingElse , syntax :: NodeOrToken :: Node (node)) => { remove . insert (node . syntax () . clone () . into ()) ; } (CfgExprStage :: EverythingElse , syntax :: NodeOrToken :: Token (token)) => { remove . insert (token . into ()) ; } _ => { } } } if stage != CfgExprStage :: EverythingElse { warn ! ("Invalid cfg_attr attribute. {:?}" , meta_tt) ; return None ; } Some (remove) }
    };
}

remove_tokens_within_cfg_attr!();