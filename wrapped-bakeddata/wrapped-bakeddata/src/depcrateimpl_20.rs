// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl DataExporter for StatisticsExporter { fn put_payload (& self , marker : DataMarkerInfo , id : DataIdentifierBorrowed , payload : & DataPayload < ExportMarker > ,) -> Result < () , DataError > { let baked_size = payload . baked_size () ; # [allow (deprecated)] use std :: hash :: { Hasher , SipHasher } ; # [allow (deprecated)] let mut hasher = SipHasher :: new () ; let postcard_size = payload . hash_and_postcard_size (& mut hasher) ; let hash = hasher . finish () ; let mut data = self . data . lock () . expect ("poison") ; let data = data . entry (marker) . or_default () ; data . size_hash . insert (id . into_owned () , ((baked_size , postcard_size) , hash)) ; data . struct_sizes . insert (hash , (baked_size , postcard_size)) ; data . identifiers . insert (id . into_owned ()) ; Ok (()) } fn close (& mut self) -> Result < ExporterCloseMetadata , DataError > { Ok (ExporterCloseMetadata (Some (Box :: new (core :: mem :: take (self . data . get_mut () . expect ("poison") ,))))) } }
};
}
