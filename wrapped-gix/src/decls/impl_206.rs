macro_rules! deps {
    () => {
        Change!();
        Error!();
        Platform!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl Change < '_ , '_ , '_ > { # [doc = " Produce a platform for performing a line-diff no matter whether the underlying [Change] is an addition, modification,"] # [doc = " deletion or rewrite."] # [doc = " Use `resource_cache` to store the diffable data and possibly reuse previously stored data, usually obtained with"] # [doc = " [Repository::diff_resource_cache()]."] # [doc = " Afterward the platform, which holds on to `resource_cache`, can be used to perform ready-made operations on the"] # [doc = " pre-set resources."] # [doc = ""] # [doc = " ### Warning about Memory Consumption"] # [doc = ""] # [doc = " `resource_cache` only grows, so one should call [`gix_diff::blob::Platform::clear_resource_cache`] occasionally."] pub fn diff < 'b > (& self , resource_cache : & 'b mut gix_diff :: blob :: Platform ,) -> Result < crate :: object :: blob :: diff :: Platform < 'b > , crate :: object :: blob :: diff :: init :: Error > { resource_cache . set_resource_by_change ((* self) . into () , & self . id () . repo . objects) ? ; Ok (crate :: object :: blob :: diff :: Platform { resource_cache }) } }
    };
}

impl_206!();