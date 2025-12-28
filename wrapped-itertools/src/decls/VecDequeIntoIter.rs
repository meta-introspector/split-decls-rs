macro_rules! VecDequeIntoIter {
    () => {
        # [cfg (feature = "use_alloc")] type VecDequeIntoIter < T > = alloc :: collections :: vec_deque :: IntoIter < T > ;
    };
}

VecDequeIntoIter!();