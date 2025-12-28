macro_rules! deps {
    () => {
        ItemLoc!();
        Struct!();
    };
}

macro_rules! StructLoc {
    () => {
        deps!();
        type StructLoc = ItemLoc < ast :: Struct > ;
    };
}

StructLoc!();