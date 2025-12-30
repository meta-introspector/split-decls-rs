// Generated macro for mark_compound (macro)
macro_rules! Depcrate_bridgemark_compound {
() => {
// Module: crate::bridge
// Provides: {"mark_compound"}
// Dependencies: {}
macro_rules ! mark_compound { (struct $ name : ident <$ ($ T : ident) ,+> { $ ($ field : ident) ,* $ (,) ? }) => { impl <$ ($ T : Mark) ,+> Mark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn mark (unmarked : Self :: Unmarked) -> Self { $ name { $ ($ field : Mark :: mark (unmarked .$ field)) ,* } } } impl <$ ($ T : Unmark) ,+> Unmark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn unmark (self) -> Self :: Unmarked { $ name { $ ($ field : Unmark :: unmark (self .$ field)) ,* } } } } ; (enum $ name : ident <$ ($ T : ident) ,+> { $ ($ variant : ident $ (($ field : ident)) ?) ,* $ (,) ? }) => { impl <$ ($ T : Mark) ,+> Mark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn mark (unmarked : Self :: Unmarked) -> Self { match unmarked { $ ($ name ::$ variant $ (($ field)) ? => { $ name ::$ variant $ ((Mark :: mark ($ field))) ? }) * } } } impl <$ ($ T : Unmark) ,+> Unmark for $ name <$ ($ T) ,+> { type Unmarked = $ name <$ ($ T :: Unmarked) ,+>; fn unmark (self) -> Self :: Unmarked { match self { $ ($ name ::$ variant $ (($ field)) ? => { $ name ::$ variant $ ((Unmark :: unmark ($ field))) ? }) * } } } } }
};
}
