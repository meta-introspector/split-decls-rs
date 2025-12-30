// Generated macro for impl_43 (impl)
macro_rules! Depcrate_grammarimpl_43 {
() => {
// Module: crate::grammar
// Provides: {"impl_43"}
// Dependencies: {}
impl Expression { fn new_kind (kind : ExpressionKind) -> Self { Self { kind , suffix : None , footnote : None , } } fn visit_nt (& self , callback : & mut dyn FnMut (& str)) { match & self . kind { ExpressionKind :: Grouped (e) | ExpressionKind :: Optional (e) | ExpressionKind :: Repeat (e) | ExpressionKind :: RepeatNonGreedy (e) | ExpressionKind :: RepeatPlus (e) | ExpressionKind :: RepeatPlusNonGreedy (e) | ExpressionKind :: RepeatRange (e , _ , _) | ExpressionKind :: NegExpression (e) => { e . visit_nt (callback) ; } ExpressionKind :: Alt (es) | ExpressionKind :: Sequence (es) => { for e in es { e . visit_nt (callback) ; } } ExpressionKind :: Nt (nt) => { callback (& nt) ; } ExpressionKind :: Terminal (_) | ExpressionKind :: Prose (_) | ExpressionKind :: Break (_) | ExpressionKind :: Comment (_) | ExpressionKind :: Unicode (_) => { } ExpressionKind :: Charset (set) => { for ch in set { match ch { Characters :: Named (s) => callback (s) , Characters :: Terminal (_) | Characters :: Range (_ , _) => { } } } } } } fn is_break (& self) -> bool { matches ! (self . kind , ExpressionKind :: Break (_)) } }
};
}
