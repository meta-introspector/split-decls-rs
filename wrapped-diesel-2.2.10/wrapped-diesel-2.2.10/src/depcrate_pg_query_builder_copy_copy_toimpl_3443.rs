// Generated macro for impl_3443 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_toimpl_3443 {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"impl_3443"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl < 'a > Row < 'a , Pg > for CopyRow < '_ > { type Field < 'f > = CopyField < 'f > where 'a : 'f , Self : 'f ; type InnerPartialRow = Self ; fn field_count (& self) -> usize { self . buffers . len () } fn get < 'b , I > (& 'b self , idx : I) -> Option < Self :: Field < 'b > > where 'a : 'b , Self : RowIndex < I > , { let idx = self . idx (idx) ? ; let buffer = self . buffers . get (idx) ? ; Some (CopyField { field : buffer , result : self . result , col_idx : idx , }) } fn partial_row (& self , range : std :: ops :: Range < usize > ,) -> row :: PartialRow < '_ , Self :: InnerPartialRow > { PartialRow :: new (self , range) } }
};
}
