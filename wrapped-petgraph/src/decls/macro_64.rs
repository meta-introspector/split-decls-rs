macro_rules! macro_64 {
    () => {
        trait_template ! { # [doc = " Base graph trait: defines the associated node identifier and"] # [doc = " edge identifier types."] pub trait GraphBase { @ escape [type NodeId] @ escape [type EdgeId] @ section nodelegate # [doc = " edge identifier"] type EdgeId : Copy + PartialEq ; # [doc = " node identifier"] type NodeId : Copy + PartialEq ; } }
    };
}

macro_64!();