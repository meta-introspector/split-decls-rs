macro_rules! ItemTreeDataStats {
    () => {
        # [derive (Default , Debug , Eq , PartialEq)] pub struct ItemTreeDataStats { pub traits : usize , pub impls : usize , pub mods : usize , pub macro_calls : usize , pub macro_rules : usize , }
    };
}

ItemTreeDataStats!()