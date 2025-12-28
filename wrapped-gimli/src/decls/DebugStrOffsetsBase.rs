macro_rules! DebugStrOffsetsBase {
    () => {
        # [doc = " An offset to a set of entries in the `.debug_str_offsets` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugStrOffsetsBase < T = usize > (pub T) ;
    };
}

DebugStrOffsetsBase!()