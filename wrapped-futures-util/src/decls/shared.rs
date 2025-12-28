macro_rules! shared {
    () => {
        # [cfg (any (feature = "std" , all (feature = "alloc" , feature = "spin")))] mod shared ;
    };
}

shared!()