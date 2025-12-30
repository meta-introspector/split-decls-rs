// Generated macro for impl_1742 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1742 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1742"}
// Dependencies: {}
impl < S : AliasSource > Alias < S > { # [doc = " Maps a single field of the source table in this alias"] pub fn field < F > (& self , field : F) -> AliasedField < S , F > where F : Column < Table = S :: Target > , { AliasedField { _alias_source : PhantomData , _field : field , } } # [doc = " Maps multiple fields of the source table in this alias"] # [doc = " (takes in tuples and some expressions)"] pub fn fields < Fields > (& self , fields : Fields) -> AliasedFields < S , Fields > where Fields : FieldAliasMapper < S > , { fields . map (self) } }
};
}
