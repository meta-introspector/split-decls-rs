macro_rules! deps {
    () => {
        ModuleId!();
    };
}

macro_rules! ItemLoc {
    () => {
        deps!();
        # [derive (Debug)] pub struct ItemLoc < N : AstIdNode > { pub container : ModuleId , pub id : AstId < N > , }
    };
}

ItemLoc!();