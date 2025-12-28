macro_rules! deps {
    () => {
        Repository!();
        Config!();
        Replace!();
        Error!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl crate :: Repository { # [doc = " Replace our own configuration with `config` and re-read all cached values, and apply them to select in-memory instances."] pub (crate) fn reread_values_and_clear_caches_replacing_config (& mut self , config : crate :: Config ,) -> Result < () , Error > { let (a , b , c) = (self . config . static_pack_cache_limit_bytes , self . config . pack_cache_bytes , self . config . object_cache_bytes ,) ; self . config . reread_values_and_clear_caches_replacing_config (config) ? ; self . apply_changed_values () ; if a != self . config . static_pack_cache_limit_bytes || b != self . config . pack_cache_bytes || c != self . config . object_cache_bytes { setup_objects (& mut self . objects , & self . config) ; } Ok (()) } fn apply_changed_values (& mut self) { self . refs . write_reflog = util :: reflog_or_default (self . config . reflog , self . workdir () . is_some ()) ; self . refs . namespace . clone_from (& self . config . refs_namespace) ; } }
    };
}

impl_529!()