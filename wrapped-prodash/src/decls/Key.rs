macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! Key {
    () => {
        deps!();
        # [doc = " A type identifying a spot in the hierarchy of `Tree` items."] # [derive (Copy , Clone , Default , Hash , Eq , PartialEq , Ord , PartialOrd , Debug)] pub struct Key (Option < Id > , Option < Id > , Option < Id > , Option < Id > , Option < Id > , Option < Id >) ;
    };
}

Key!()