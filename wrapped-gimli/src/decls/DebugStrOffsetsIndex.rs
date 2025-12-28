macro_rules! DebugStrOffsetsIndex {
    () => {
        # [doc = " An index into a set of entries in the `.debug_str_offsets` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugStrOffsetsIndex < T = usize > (pub T) ;
    };
}

DebugStrOffsetsIndex!();