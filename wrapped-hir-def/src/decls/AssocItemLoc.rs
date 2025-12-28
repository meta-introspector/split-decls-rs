macro_rules! deps {
    () => {
        ItemContainerId!();
    };
}

macro_rules! AssocItemLoc {
    () => {
        deps!();
        # [derive (Debug)] pub struct AssocItemLoc < N : AstIdNode > { pub container : ItemContainerId , pub id : AstId < N > , }
    };
}

AssocItemLoc!();