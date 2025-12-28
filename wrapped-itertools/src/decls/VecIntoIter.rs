macro_rules! VecIntoIter {
    () => {
        # [cfg (feature = "use_alloc")] type VecIntoIter < T > = alloc :: vec :: IntoIter < T > ;
    };
}

VecIntoIter!();