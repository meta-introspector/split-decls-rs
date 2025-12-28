macro_rules! deps {
    () => {
        PartitionMap!();
        ParallelExtend!();
        ParallelIterator!();
    };
}

macro_rules! partition_map {
    () => {
        deps!();
        # [doc = " Partitions and maps the items of a parallel iterator into a pair of"] # [doc = " arbitrary `ParallelExtend` containers."] # [doc = ""] # [doc = " This called by `ParallelIterator::partition_map`."] pub (super) fn partition_map < I , A , B , P , L , R > (pi : I , predicate : P) -> (A , B) where I : ParallelIterator , A : Default + Send + ParallelExtend < L > , B : Default + Send + ParallelExtend < R > , P : Fn (I :: Item) -> Either < L , R > + Sync + Send , L : Send , R : Send , { execute (pi , PartitionMap { predicate }) }
    };
}

partition_map!()