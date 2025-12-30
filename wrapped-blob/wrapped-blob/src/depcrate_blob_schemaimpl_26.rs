// Generated macro for impl_26 (impl)
macro_rules! Depcrate_blob_schemaimpl_26 {
() => {
// Module: crate::blob_schema
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'data > BlobSchema < 'data > { pub fn deserialize_and_check < D : serde :: Deserializer < 'data > > (de : D ,) -> Result < BlobSchema < 'data > , D :: Error > { let blob = Self :: deserialize (de) ? ; # [cfg (debug_assertions)] blob . check_invariants () ; Ok (blob) } pub fn load (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < (& 'data [u8] , Option < u64 >) , DataError > { match self { BlobSchema :: V001 (..) | BlobSchema :: V002 (..) | BlobSchema :: V002Bigger (..) => { unreachable ! ("Unreachable blob schema") } BlobSchema :: V003 (s) => s . load (marker , req) , BlobSchema :: V003Bigger (s) => s . load (marker , req) , } } # [cfg (feature = "alloc")] pub fn iter_ids (& self , marker : DataMarkerInfo ,) -> Result < alloc :: collections :: BTreeSet < DataIdentifierCow < '_ > > , DataError > { match self { BlobSchema :: V001 (..) | BlobSchema :: V002 (..) | BlobSchema :: V002Bigger (..) => { unreachable ! ("Unreachable blob schema") } BlobSchema :: V003 (s) => s . iter_ids (marker) , BlobSchema :: V003Bigger (s) => s . iter_ids (marker) , } } # [cfg (debug_assertions)] fn check_invariants (& self) { match self { BlobSchema :: V001 (..) | BlobSchema :: V002 (..) | BlobSchema :: V002Bigger (..) => () , BlobSchema :: V003 (s) => s . check_invariants () , BlobSchema :: V003Bigger (s) => s . check_invariants () , } } }
};
}
