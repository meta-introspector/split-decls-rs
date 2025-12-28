macro_rules! PathTracker {
    () => {
        struct PathTracker < G > where G : GraphBase , G :: NodeId : Eq + Hash , { came_from : HashMap < G :: NodeId , G :: NodeId > , }
    };
}

PathTracker!()