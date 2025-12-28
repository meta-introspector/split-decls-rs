macro_rules! deps {
    () => {
        Mod!();
        Use!();
        ExternCrate!();
    };
}

macro_rules! BigModItem {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] enum BigModItem { ExternCrate (ExternCrate) , Mod (Mod) , Use (Use) , }
    };
}

BigModItem!()