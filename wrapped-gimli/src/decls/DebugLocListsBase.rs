macro_rules! DebugLocListsBase {
    () => {
        # [doc = " An offset to a set of location list offsets in the `.debug_loclists` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugLocListsBase < T = usize > (pub T) ;
    };
}

DebugLocListsBase!()