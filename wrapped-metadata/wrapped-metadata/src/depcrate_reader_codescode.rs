// Generated macro for code (macro)
macro_rules! Depcrate_reader_codescode {
() => {
// Module: crate::reader::codes
// Provides: {"code"}
// Dependencies: {}
macro_rules ! code { ($ name : ident ($ size : literal) $ (($ table : ident , $ code : literal)) +) => { # [derive (Clone , Debug , Hash , PartialEq , Eq , Ord , PartialOrd)] pub enum $ name <'a > { $ ($ table ($ table <'a >) ,) * } impl <'a > Decode <'a > for $ name <'a > { fn decode (index : &'a TypeIndex , file : usize , code : usize) -> Self { let (kind , row) = (code & ((1 << $ size) - 1) , (code >> $ size) - 1) ; match kind { $ ($ code => Self ::$ table ($ table (Row :: new (index , file , row))) ,) * rest => panic ! ("{rest:?}") , } } } impl $ name <'_ > { # [allow (dead_code)] pub fn encode (& self) -> usize { match self { $ (Self ::$ table (row) => (row . pos () + 1) << $ size | $ code ,) * } } } $ (impl <'a > From <$ table <'a >> for $ name <'a > { fn from (from : $ table <'a >) -> Self { Self ::$ table (from) } }) * } ; }
};
}
