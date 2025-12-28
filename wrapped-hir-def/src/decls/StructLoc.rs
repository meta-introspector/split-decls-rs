macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! StructLoc {
    () => {
        deps!();
        type StructLoc = ItemLoc < ast :: Struct > ;
    };
}

StructLoc!()