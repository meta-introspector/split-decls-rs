macro_rules! deps {
    () => {
        Attrs!();
        AttrDefId!();
    };
}

macro_rules! AttrsWithOwner {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct AttrsWithOwner { attrs : Attrs , owner : AttrDefId , }
    };
}

AttrsWithOwner!();