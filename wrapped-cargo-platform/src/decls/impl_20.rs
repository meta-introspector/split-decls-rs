macro_rules! deps {
    () => {
        Token!();
        Cfg!();
        Parser!();
        Ident!();
        Tokenizer!();
        ParseError!();
        CfgExpr!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'a > Parser < 'a > { fn new (s : & 'a str) -> Parser < 'a > { Parser { t : Tokenizer { s : s . char_indices () . peekable () , orig : s , } , } } fn expr (& mut self) -> Result < CfgExpr , ParseError > { match self . peek () { Some (Ok (Token :: Ident (false , op @ "all"))) | Some (Ok (Token :: Ident (false , op @ "any"))) => { self . t . next () ; let mut e = Vec :: new () ; self . eat (& Token :: LeftParen) ? ; while ! self . r#try (& Token :: RightParen) { e . push (self . expr () ?) ; if ! self . r#try (& Token :: Comma) { self . eat (& Token :: RightParen) ? ; break ; } } if op == "all" { Ok (CfgExpr :: All (e)) } else { Ok (CfgExpr :: Any (e)) } } Some (Ok (Token :: Ident (false , "not"))) => { self . t . next () ; self . eat (& Token :: LeftParen) ? ; let e = self . expr () ? ; self . eat (& Token :: RightParen) ? ; Ok (CfgExpr :: Not (Box :: new (e))) } Some (Ok (..)) => self . cfg () . map (| v | match v { Cfg :: Name (n) if n == "true" => CfgExpr :: True , Cfg :: Name (n) if n == "false" => CfgExpr :: False , v => CfgExpr :: Value (v) , }) , Some (Err (..)) => Err (self . t . next () . unwrap () . err () . unwrap ()) , None => Err (ParseError :: new (self . t . orig , IncompleteExpr ("start of a cfg expression") ,)) , } } fn cfg (& mut self) -> Result < Cfg , ParseError > { match self . t . next () { Some (Ok (Token :: Ident (raw , name))) => { let e = if self . r#try (& Token :: Equals) { let val = match self . t . next () { Some (Ok (Token :: String (s))) => s , Some (Ok (t)) => { return Err (ParseError :: new (self . t . orig , UnexpectedToken { expected : "a string" , found : t . classify () , } ,)) ; } Some (Err (e)) => return Err (e) , None => { return Err (ParseError :: new (self . t . orig , IncompleteExpr ("a string"))) ; } } ; Cfg :: KeyPair (Ident { name : name . to_string () , raw , } , val . to_string () ,) } else { Cfg :: Name (Ident { name : name . to_string () , raw , }) } ; Ok (e) } Some (Ok (t)) => Err (ParseError :: new (self . t . orig , UnexpectedToken { expected : "identifier" , found : t . classify () , } ,)) , Some (Err (e)) => Err (e) , None => Err (ParseError :: new (self . t . orig , IncompleteExpr ("identifier"))) , } } fn peek (& mut self) -> Option < Result < Token < 'a > , ParseError > > { self . t . clone () . next () } fn r#try (& mut self , token : & Token < 'a >) -> bool { match self . peek () { Some (Ok (ref t)) if token == t => { } _ => return false , } self . t . next () ; true } fn eat (& mut self , token : & Token < 'a >) -> Result < () , ParseError > { match self . t . next () { Some (Ok (ref t)) if token == t => Ok (()) , Some (Ok (t)) => Err (ParseError :: new (self . t . orig , UnexpectedToken { expected : token . classify () , found : t . classify () , } ,)) , Some (Err (e)) => Err (e) , None => Err (ParseError :: new (self . t . orig , IncompleteExpr (token . classify ()) ,)) , } } # [doc = " Returns the rest of the input from the current location."] fn rest (& self) -> Option < & str > { let mut s = self . t . s . clone () ; loop { match s . next () { Some ((_ , ' ')) => { } Some ((start , _ch)) => return Some (& self . t . orig [start ..]) , None => return None , } } } }
    };
}

impl_20!();