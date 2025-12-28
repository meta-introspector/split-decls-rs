macro_rules! Memmem {
    () => {
        # [derive (Clone , Debug)] pub (crate) struct Memmem { # [cfg (not (all (feature = "std" , feature = "perf-literal-substring")))] _unused : () , # [cfg (all (feature = "std" , feature = "perf-literal-substring"))] finder : memchr :: memmem :: Finder < 'static > , }
    };
}

Memmem!();