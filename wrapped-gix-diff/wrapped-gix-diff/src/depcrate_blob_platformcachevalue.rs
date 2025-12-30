// Generated macro for CacheValue (struct)
macro_rules! Depcrate_blob_platformCacheValue {
() => {
// Module: crate::blob::platform
// Provides: {"CacheValue"}
// Dependencies: {}
# [doc = " A stored value representing a diffable resource."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] pub (crate) struct CacheValue { # [doc = " The outcome of converting a resource into a diffable format using [Pipeline::convert_to_diffable()]."] conversion : pipeline :: Outcome , # [doc = " The kind of the resource we are looking at. Only possible values are `Blob`, `BlobExecutable` and `Link`."] mode : gix_object :: tree :: EntryKind , # [doc = " A possibly empty buffer, depending on `conversion.data` which may indicate the data is considered binary."] buffer : Vec < u8 > , }
};
}
