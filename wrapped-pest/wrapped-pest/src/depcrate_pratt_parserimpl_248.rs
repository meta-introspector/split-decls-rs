// Generated macro for impl_248 (impl)
macro_rules! Depcrate_pratt_parserimpl_248 {
() => {
// Module: crate::pratt_parser
// Provides: {"impl_248"}
// Dependencies: {}
impl < R : RuleType > Op < R > { # [doc = " Defines `rule` as a prefix unary operator."] pub fn prefix (rule : R) -> Self { Self { rule , affix : Affix :: Prefix , next : None , } } # [doc = " Defines `rule` as a postfix unary operator."] pub fn postfix (rule : R) -> Self { Self { rule , affix : Affix :: Postfix , next : None , } } # [doc = " Defines `rule` as an infix binary operator with associativity `assoc`."] pub fn infix (rule : R , assoc : Assoc) -> Self { Self { rule , affix : Affix :: Infix (assoc) , next : None , } } }
};
}
