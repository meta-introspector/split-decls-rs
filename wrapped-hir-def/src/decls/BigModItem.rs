macro_rules! deps {
    () => {
        Use!();
        Mod!();
        ExternCrate!();
    };
}

macro_rules! BigModItem {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] enum BigModItem { ExternCrate (ExternCrate) , Mod (Mod) , Use (Use) , }
    };
}

BigModItem!();