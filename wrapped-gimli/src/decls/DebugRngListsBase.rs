macro_rules! DebugRngListsBase {
    () => {
        # [doc = " An offset to a set of range list offsets in the `.debug_rnglists` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugRngListsBase < T = usize > (pub T) ;
    };
}

DebugRngListsBase!();