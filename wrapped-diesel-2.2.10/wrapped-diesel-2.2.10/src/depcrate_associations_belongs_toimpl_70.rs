// Generated macro for impl_70 (impl)
macro_rules! Depcrate_associations_belongs_toimpl_70 {
() => {
// Module: crate::associations::belongs_to
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'a , Parent , Child > BelongingToDsl < & 'a Parent > for Child where & 'a Parent : Identifiable , Child : HasTable + BelongsTo < Parent > , Id < & 'a Parent > : AsExpression < < Child :: ForeignKeyColumn as Expression > :: SqlType > , Child :: Table : FilterDsl < Eq < Child :: ForeignKeyColumn , Id < & 'a Parent > > > , Child :: ForeignKeyColumn : ExpressionMethods , < Child :: ForeignKeyColumn as Expression > :: SqlType : SqlType , { type Output = FindBy < Child :: Table , Child :: ForeignKeyColumn , Id < & 'a Parent > > ; fn belonging_to (parent : & 'a Parent) -> Self :: Output { FilterDsl :: filter (Child :: table () , Child :: foreign_key_column () . eq (parent . id ())) } }
};
}
