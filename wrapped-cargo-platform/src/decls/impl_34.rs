macro_rules! deps {
    () => {
        Platform!();
        CfgExpr!();
        ParseErrorKind!();
        ParseError!();
        Cfg!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Platform { # [doc = " Returns whether the Platform matches the given target and cfg."] # [doc = ""] # [doc = " The named target and cfg values should be obtained from `rustc`."] pub fn matches (& self , name : & str , cfg : & [Cfg]) -> bool { match * self { Platform :: Name (ref p) => p == name , Platform :: Cfg (ref p) => p . matches (cfg) , } } fn validate_named_platform (name : & str) -> Result < () , ParseError > { if let Some (ch) = name . chars () . find (| & c | ! (c . is_alphanumeric () || c == '_' || c == '-' || c == '.')) { if name . chars () . any (| c | c == '(') { return Err (ParseError :: new (name , ParseErrorKind :: InvalidTarget ("unexpected `(` character, cfg expressions must start with `cfg(`" . to_string () ,) ,)) ; } return Err (ParseError :: new (name , ParseErrorKind :: InvalidTarget (format ! ("unexpected character {} in target name" , ch)) ,)) ; } Ok (()) } pub fn check_cfg_attributes (& self , warnings : & mut Vec < String >) { fn check_cfg_expr (expr : & CfgExpr , warnings : & mut Vec < String >) { match * expr { CfgExpr :: Not (ref e) => check_cfg_expr (e , warnings) , CfgExpr :: All (ref e) | CfgExpr :: Any (ref e) => { for e in e { check_cfg_expr (e , warnings) ; } } CfgExpr :: Value (ref e) => match e { Cfg :: Name (name) => match name . as_str () { "test" | "debug_assertions" | "proc_macro" => warnings . push (format ! ("Found `{}` in `target.'cfg(...)'.dependencies`. \
                                 This value is not supported for selecting dependencies \
                                 and will not work as expected. \
                                 To learn more visit \
                                 https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies" , name)) , _ => () , } , Cfg :: KeyPair (name , _) => if name . as_str () == "feature" { warnings . push (String :: from ("Found `feature = ...` in `target.'cfg(...)'.dependencies`. \
                             This key is not supported for selecting dependencies \
                             and will not work as expected. \
                             Use the [features] section instead: \
                             https://doc.rust-lang.org/cargo/reference/features.html")) } , } CfgExpr :: True | CfgExpr :: False => { } , } } if let Platform :: Cfg (cfg) = self { check_cfg_expr (cfg , warnings) ; } } pub fn check_cfg_keywords (& self , warnings : & mut Vec < String > , path : & Path) { fn check_cfg_expr (expr : & CfgExpr , warnings : & mut Vec < String > , path : & Path) { match * expr { CfgExpr :: Not (ref e) => check_cfg_expr (e , warnings , path) , CfgExpr :: All (ref e) | CfgExpr :: Any (ref e) => { for e in e { check_cfg_expr (e , warnings , path) ; } } CfgExpr :: True | CfgExpr :: False => { } CfgExpr :: Value (ref e) => match e { Cfg :: Name (name) | Cfg :: KeyPair (name , _) => { if ! name . raw && KEYWORDS . contains (& name . as_str ()) { warnings . push (format ! ("[{}] future-incompatibility: `cfg({e})` is deprecated as `{name}` is a keyword \
                                 and not an identifier and should not have have been accepted in this position.\n \
                                 | this was previously accepted by Cargo but is being phased out; it will become a hard error in a future release!\n \
                                 |\n \
                                 | help: use raw-idents instead: `cfg(r#{name})`" , path . display ())) ; } } } , } } if let Platform :: Cfg (cfg) = self { check_cfg_expr (cfg , warnings , path) ; } } }
    };
}

impl_34!()