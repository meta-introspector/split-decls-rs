macro_rules! deps {
    () => {
        Rule!();
        Expr!();
    };
}

macro_rules! OptimizedExpr {
    () => {
        deps!();
        # [doc = " The optimized version of the pest AST's `Expr`."] # [doc = ""] # [doc = " # Warning: Semantic Versioning"] # [doc = " There may be non-breaking changes to the meta-grammar"] # [doc = " between minor versions. Those non-breaking changes, however,"] # [doc = " may translate into semver-breaking changes due to the additional variants"] # [doc = " propagated from the `Rule` enum. This is a known issue and will be fixed in the"] # [doc = " future (e.g. by increasing MSRV and non_exhaustive annotations)."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum OptimizedExpr { # [doc = " Matches an exact string, e.g. `\"a\"`"] Str (String) , # [doc = " Matches an exact string, case insensitively (ASCII only), e.g. `^\"a\"`"] Insens (String) , # [doc = " Matches one character in the range, e.g. `'a'..'z'`"] Range (String , String) , # [doc = " Matches the rule with the given name, e.g. `a`"] Ident (String) , # [doc = " Matches a custom part of the stack, e.g. `PEEK[..]`"] PeekSlice (i32 , Option < i32 >) , # [doc = " Positive lookahead; matches expression without making progress, e.g. `&e`"] PosPred (Box < OptimizedExpr >) , # [doc = " Negative lookahead; matches if expression doesn't match, without making progress, e.g. `!e`"] NegPred (Box < OptimizedExpr >) , # [doc = " Matches a sequence of two expressions, e.g. `e1 ~ e2`"] Seq (Box < OptimizedExpr > , Box < OptimizedExpr >) , # [doc = " Matches either of two expressions, e.g. `e1 | e2`"] Choice (Box < OptimizedExpr > , Box < OptimizedExpr >) , # [doc = " Optionally matches an expression, e.g. `e?`"] Opt (Box < OptimizedExpr >) , # [doc = " Matches an expression zero or more times, e.g. `e*`"] Rep (Box < OptimizedExpr >) , # [doc = " Matches an expression one or more times, e.g. `e+`"] # [cfg (feature = "grammar-extras")] RepOnce (Box < OptimizedExpr >) , # [doc = " Continues to match expressions until one of the strings in the `Vec` is found"] Skip (Vec < String >) , # [doc = " Matches an expression and pushes it to the stack, e.g. `push(e)`"] Push (Box < OptimizedExpr >) , # [doc = " Pushes a literal string to the stack, e.g. `push_literal(\"a\")`"] # [cfg (feature = "grammar-extras")] PushLiteral (String) , # [doc = " Matches an expression and assigns a label to it, e.g. #label = exp"] # [cfg (feature = "grammar-extras")] NodeTag (Box < OptimizedExpr > , String) , # [doc = " Restores an expression's checkpoint"] RestoreOnErr (Box < OptimizedExpr >) , }
    };
}

OptimizedExpr!();