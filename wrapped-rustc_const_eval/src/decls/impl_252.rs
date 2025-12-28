macro_rules! deps {
    () => {
        Machine!();
        Memory!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < 'tcx , M : Machine < 'tcx > > Memory < 'tcx , M > { pub fn new () -> Self { Memory { alloc_map : M :: MemoryMap :: default () , extra_fn_ptr_map : FxIndexMap :: default () , dead_alloc_map : FxIndexMap :: default () , validation_in_progress : Cell :: new (false) , } } # [doc = " This is used by [priroda](https://github.com/oli-obk/priroda)"] pub fn alloc_map (& self) -> & M :: MemoryMap { & self . alloc_map } }
    };
}

impl_252!()