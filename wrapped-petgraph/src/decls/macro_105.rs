macro_rules! deps {
    () => {
        Create!();
        VisitMap!();
    };
}

macro_rules! macro_105 {
    () => {
        deps!();
        trait_template ! { # [doc = " A graph that can create a map that tracks the visited status of its nodes."] # [allow (clippy :: needless_arbitrary_self_type)] pub trait Visitable : GraphBase { @ section type # [doc = " The associated map type"] type Map : VisitMap < Self :: NodeId >; @ section self # [doc = " Create a new visitor map"] fn visit_map (self : & Self) -> Self :: Map ; # [doc = " Reset the visitor map (and resize to new size of graph if needed)"] fn reset_map (self : & Self , map : & mut Self :: Map) ; } }
    };
}

macro_105!()