// Generated macro for impl_507 (impl)
macro_rules! Depcrate_serdeimpl_507 {
() => {
// Module: crate::serde
// Provides: {"impl_507"}
// Dependencies: {}
impl < T > serde :: de :: Visitor < '_ > for ParseVisitor < T > where T : FromStr , < T as FromStr > :: Err : Display , { type Value = T ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (formatter , "a valid Unicode Language or Locale Identifier") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { s . parse :: < T > () . map_err (serde :: de :: Error :: custom) } }
};
}
