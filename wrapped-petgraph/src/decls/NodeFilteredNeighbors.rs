macro_rules! NodeFilteredNeighbors {
    () => {
        # [doc = " A filtered neighbors iterator."] # [derive (Debug , Clone)] pub struct NodeFilteredNeighbors < 'a , I , F : 'a > { include_source : bool , iter : I , f : & 'a F , }
    };
}

NodeFilteredNeighbors!();