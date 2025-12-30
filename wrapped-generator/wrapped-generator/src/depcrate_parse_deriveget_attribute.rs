// Generated macro for get_attribute (function)
macro_rules! Depcrate_parse_deriveget_attribute {
() => {
// Module: crate::parse_derive
// Provides: {"get_attribute"}
// Dependencies: {}
fn get_attribute (attr : & Attribute) -> GrammarSource { match & attr . meta { Meta :: NameValue (name_value) => match & name_value . value { Expr :: Lit (ExprLit { lit : Lit :: Str (string) , .. }) => { if name_value . path . is_ident ("grammar") { GrammarSource :: File (string . value ()) } else { GrammarSource :: Inline (string . value ()) } } _ => panic ! ("grammar attribute must be a string") , } , _ => panic ! ("grammar attribute must be of the form `grammar = \"...\"`") , } }
};
}
