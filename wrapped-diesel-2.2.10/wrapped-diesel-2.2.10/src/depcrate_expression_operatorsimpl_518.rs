// Generated macro for impl_518 (impl)
macro_rules! Depcrate_expression_operatorsimpl_518 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_518"}
// Dependencies: {}
impl < S , T , U > crate :: internal :: operators_macro :: FieldAliasMapper < S > for Like < T , U > where S : crate :: query_source :: AliasSource , T : crate :: internal :: operators_macro :: FieldAliasMapper < S > , U : crate :: internal :: operators_macro :: FieldAliasMapper < S > , { type Out = Like < < T as crate :: internal :: operators_macro :: FieldAliasMapper < S > > :: Out , < U as crate :: internal :: operators_macro :: FieldAliasMapper < S > > :: Out , > ; fn map (self , alias : & crate :: query_source :: Alias < S >) -> Self :: Out { Like { left : self . left . map (alias) , right : self . right . map (alias) , } } }
};
}
