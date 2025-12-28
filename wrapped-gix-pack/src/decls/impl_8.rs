macro_rules! deps {
    () => {
        Options!();
        Mode!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Default for Options { # [doc = " Options which favor speed and correctness and write the most commonly supported index file."] fn default () -> Self { Options { thread_limit : None , iteration_mode : crate :: data :: input :: Mode :: Verify , index_version : Default :: default () , object_hash : Default :: default () , } } }
    };
}

impl_8!();