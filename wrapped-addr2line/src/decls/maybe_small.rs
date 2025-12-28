macro_rules! maybe_small {
    () => {
        # [cfg (not (feature = "smallvec"))] mod maybe_small { pub type Vec < T > = alloc :: vec :: Vec < T > ; pub type IntoIter < T > = alloc :: vec :: IntoIter < T > ; }
    };
}

maybe_small!()