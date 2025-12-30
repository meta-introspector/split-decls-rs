// Generated macro for impl_72 (impl)
macro_rules! Depcrate_associations_belongs_toimpl_72 {
() => {
// Module: crate::associations::belongs_to
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a , Parent , Child > BelongingToDsl < (& 'a [Parent] , & 'a [Parent]) > for Child where & 'a Parent : Identifiable , Child : HasTable + BelongsTo < Parent > , Vec < Id < & 'a Parent > > : AsInExpression < < Child :: ForeignKeyColumn as Expression > :: SqlType > , < Child as HasTable > :: Table : FilterDsl < EqAny < Child :: ForeignKeyColumn , Vec < Id < & 'a Parent > > > > , Child :: ForeignKeyColumn : ExpressionMethods , < Child :: ForeignKeyColumn as Expression > :: SqlType : SqlType , { type Output = Filter < Child :: Table , EqAny < Child :: ForeignKeyColumn , Vec < Id < & 'a Parent > > > > ; fn belonging_to (parents : (& 'a [Parent] , & 'a [Parent])) -> Self :: Output { let ids = parents . 0 . iter () . chain (parents . 1 . iter ()) . map (Identifiable :: id) . collect :: < Vec < _ > > () ; FilterDsl :: filter (Child :: table () , Child :: foreign_key_column () . eq_any (ids)) } }
};
}
