macro_rules! primitive {
    () => {
        # [cfg (not (crossbeam_loom))] # [allow (unused_imports)] mod primitive { pub (crate) mod hint { pub (crate) use core :: hint :: spin_loop ; } pub (crate) mod sync { pub (crate) use core :: sync :: atomic ; # [cfg (feature = "std")] pub (crate) use std :: sync :: { Arc , Condvar , Mutex } ; } }
    };
}

primitive!();