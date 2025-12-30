// Generated macro for op_utils (macro)
macro_rules! Depcrateop_utils {
() => {
// Module: crate
// Provides: {"op_utils"}
// Dependencies: {}
macro_rules ! op_utils { ($ ($ name : ident $ assign : ident) *) => { # [doc = " Binary operation traits like `LangItem::Add`"] pub static BINOP_TRAITS : & [LangItem] = & [$ (LangItem ::$ name ,) *] ; # [doc = " Operator-Assign traits like `LangItem::AddAssign`"] pub static OP_ASSIGN_TRAITS : & [LangItem] = & [$ (LangItem ::$ assign ,) *] ; # [doc = " Converts `BinOpKind::Add` to `(LangItem::Add, LangItem::AddAssign)`, for example"] pub fn binop_traits (kind : hir :: BinOpKind) -> Option < (LangItem , LangItem) > { match kind { $ (hir :: BinOpKind ::$ name => Some ((LangItem ::$ name , LangItem ::$ assign)) ,) * _ => None , } } } ; }
};
}
