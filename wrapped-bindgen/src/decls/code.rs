macro_rules! deps {
    () => {
        File!();
        Row!();
        Decode!();
    };
}

macro_rules! code {
    () => {
        deps!();
        macro_rules ! code { ($ name : ident ($ size : literal) $ (($ table : ident , $ code : literal)) +) => { # [derive (Clone , Debug , Hash , PartialEq , Eq , Ord , PartialOrd)] pub enum $ name { $ ($ table ($ table) ,) * } impl Decode for $ name { fn decode (file : &'static File , code : usize) -> Self { let (kind , row) = (code & ((1 << $ size) - 1) , (code >> $ size) - 1) ; match kind { $ ($ code => Self ::$ table ($ table (Row :: new (file , row))) ,) * rest => panic ! ("{rest:?}") , } } } impl $ name { # [allow (dead_code)] pub fn encode (& self) -> usize { match self { $ (Self ::$ table (row) => (row . index () + 1) << $ size | $ code ,) * } } } $ (impl From <$ table > for $ name { fn from (from : $ table) -> Self { Self ::$ table (from) } }) * } ; }
    };
}

code!();