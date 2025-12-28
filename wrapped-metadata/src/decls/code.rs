macro_rules! code {
    () => {
        macro_rules ! code { ($ name : ident ($ size : literal) $ (($ table : ident , $ code : literal)) +) => { # [derive (Clone , Copy , Eq , PartialEq , Hash)] pub enum $ name { $ ($ table (id ::$ table) ,) * } impl $ name { pub fn encode (& self) -> u32 { match self { $ (Self ::$ table (row) => (row . 0 . overflowing_add (1) . 0) << $ size | $ code ,) * } } } impl Ord for $ name { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . encode () . cmp (& other . encode ()) } } impl PartialOrd for $ name { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } } } ; }
    };
}

code!();