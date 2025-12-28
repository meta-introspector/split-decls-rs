macro_rules! PoolGuard {
    () => {
        # [doc = " A guard that is returned when a caller requests a value from the pool."] # [doc = ""] # [doc = " The purpose of the guard is to use RAII to automatically put the value"] # [doc = " back in the pool once it's dropped."] pub struct PoolGuard < 'a , T : Send , F : Fn () -> T > (inner :: PoolGuard < 'a , T , F >) ;
    };
}

PoolGuard!()