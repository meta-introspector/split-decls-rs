macro_rules! ArenaMap {
    () => {
        # [doc = " A map from arena indexes to some other type."] # [doc = " Space requirement is O(highest index)."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct ArenaMap < IDX , V > { v : Vec < Option < V > > , _ty : PhantomData < IDX > , }
    };
}

ArenaMap!();