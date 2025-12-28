macro_rules! deps {
    () => {
        SiblingLocation!();
    };
}

macro_rules! Adjacency {
    () => {
        deps!();
        # [doc = " A type providing information about what's above and below `Tree` items."] # [derive (Copy , Clone , Default , Eq , PartialEq , Ord , PartialOrd , Debug)] pub struct Adjacency (pub SiblingLocation , pub SiblingLocation , pub SiblingLocation , pub SiblingLocation , pub SiblingLocation , pub SiblingLocation ,) ;
    };
}

Adjacency!()