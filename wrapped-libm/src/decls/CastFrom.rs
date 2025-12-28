macro_rules! CastFrom {
    () => {
        pub trait CastFrom < T : Copy > : Copy { # [doc = " By default, casts should be exact."] # [track_caller] fn cast_from (value : T) -> Self ; # [doc = " Call for casts that are expected to truncate."] fn cast_from_lossy (value : T) -> Self ; }
    };
}

CastFrom!();