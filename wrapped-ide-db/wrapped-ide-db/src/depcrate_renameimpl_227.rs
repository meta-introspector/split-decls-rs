// Generated macro for impl_227 (impl)
macro_rules! Depcrate_renameimpl_227 {
() => {
// Module: crate::rename
// Provides: {"impl_227"}
// Dependencies: {}
impl IdentifierKind { pub fn classify (edition : Edition , new_name : & str) -> Result < (Name , IdentifierKind) > { match parser :: LexedStr :: single_token (edition , new_name) { Some (res) => match res { (SyntaxKind :: IDENT , _) => Ok ((Name :: new_root (new_name) , IdentifierKind :: Ident)) , (T ! [_] , _) => { Ok ((Name :: new_symbol_root (sym :: underscore) , IdentifierKind :: Underscore)) } (SyntaxKind :: LIFETIME_IDENT , _) if new_name != "'static" && new_name != "'_" => { Ok ((Name :: new_lifetime (new_name) , IdentifierKind :: Lifetime)) } _ if SyntaxKind :: from_keyword (new_name , edition) . is_some () => match new_name { "self" => Ok ((Name :: new_root (new_name) , IdentifierKind :: LowercaseSelf)) , "crate" | "super" | "Self" => { bail ! ("Invalid name `{}`: cannot rename to a keyword" , new_name) } _ => Ok ((Name :: new_root (new_name) , IdentifierKind :: Ident)) , } , (_ , Some (syntax_error)) => bail ! ("Invalid name `{}`: {}" , new_name , syntax_error) , (_ , None) => bail ! ("Invalid name `{}`: not an identifier" , new_name) , } , None => bail ! ("Invalid name `{}`: not an identifier" , new_name) , } } }
};
}
