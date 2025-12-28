macro_rules! deps {
    () => {
        DefaultRandomSource!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl DefaultRandomSource { fn new () -> DefaultRandomSource { DefaultRandomSource { counter : AtomicUsize :: new (& PI_U64X4 as * const _ as usize) , } } # [cfg (all (target_arch = "arm" , target_os = "none"))] const fn default () -> DefaultRandomSource { DefaultRandomSource { counter : AtomicUsize :: new (PI_U64X4 [3] as usize) , } } }
    };
}

impl_77!()