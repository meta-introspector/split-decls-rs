macro_rules! MayLeak {
    () => {
        # [doc = " Whether this kind of memory is allowed to leak"] pub trait MayLeak : Copy { fn may_leak (self) -> bool ; }
    };
}

MayLeak!()