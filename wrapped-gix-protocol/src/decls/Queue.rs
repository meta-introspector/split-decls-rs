macro_rules! Queue {
    () => {
        type Queue = gix_revwalk :: PriorityQueue < SecondsSinceUnixEpoch , gix_hash :: ObjectId > ;
    };
}

Queue!()