// Generated macro for impl_71 (impl)
macro_rules! Depcrate_associations_belongs_toimpl_71 {
() => {
// Module: crate::associations::belongs_to
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'a , Parent , Child > BelongingToDsl < & 'a [Parent] > for Child where & 'a Parent : Identifiable , Child : HasTable + BelongsTo < Parent > , Vec < Id < & 'a Parent > > : AsInExpression < < Child :: ForeignKeyColumn as Expression > :: SqlType > , < Child as HasTable > :: Table : FilterDsl < EqAny < Child :: ForeignKeyColumn , Vec < Id < & 'a Parent > > > > , Child :: ForeignKeyColumn : ExpressionMethods , < Child :: ForeignKeyColumn as Expression > :: SqlType : SqlType , { type Output = Filter < Child :: Table , EqAny < Child :: ForeignKeyColumn , Vec < Id < & 'a Parent > > > > ; fn belonging_to (parents : & 'a [Parent]) -> Self :: Output { let ids = parents . iter () . map (Identifiable :: id) . collect :: < Vec < _ > > () ; FilterDsl :: filter (Child :: table () , Child :: foreign_key_column () . eq_any (ids)) } }
};
}
