macro_rules! deps {
    () => {
        RandomState!();
        RandomSource!();
        DefaultRandomSource!();
    };
}

macro_rules! macro_79 {
    () => {
        deps!();
        cfg_if :: cfg_if ! { if # [cfg (all (target_arch = "arm" , target_os = "none"))] { # [inline] fn get_src () -> &'static dyn RandomSource { static RAND_SOURCE : DefaultRandomSource = DefaultRandomSource :: default () ; & RAND_SOURCE } } else { # [doc = " Provides an optional way to manually supply a source of randomness for Hasher keys."] # [doc = ""] # [doc = " The provided [RandomSource] will be used to be used as a source of randomness by [RandomState] to generate new states."] # [doc = " If this method is not invoked the standard source of randomness is used as described in the Readme."] # [doc = ""] # [doc = " The source of randomness can only be set once, and must be set before the first RandomState is created."] # [doc = " If the source has already been specified `Err` is returned with a `bool` indicating if the set failed because"] # [doc = " method was previously invoked (true) or if the default source is already being used (false)."] # [cfg (not (all (target_arch = "arm" , target_os = "none")))] pub fn set_random_source (source : impl RandomSource + Send + Sync + 'static) -> Result < () , bool > { RAND_SOURCE . set (Box :: new (Box :: new (source))) . map_err (| s | s . as_ref () . type_id () != TypeId :: of ::<& DefaultRandomSource > ()) } # [inline] fn get_src () -> &'static dyn RandomSource { RAND_SOURCE . get_or_init (|| Box :: new (Box :: new (DefaultRandomSource :: new ()))) . as_ref () } } }
    };
}

macro_79!()