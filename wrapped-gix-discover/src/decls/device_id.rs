macro_rules! device_id {
    () => {
        # [doc = " Returns the device ID of the directory."] # [cfg (all (unix , not (target_os = "linux")))] pub (crate) fn device_id (m : & std :: fs :: Metadata) -> u64 { use std :: os :: unix :: fs :: MetadataExt ; m . dev () }
    };
}

device_id!();