macro_rules! HasStaticRootDefId {
    () => {
        pub trait HasStaticRootDefId { # [doc = " Returns the `DefId` of the static item that is currently being evaluated."] # [doc = " Used for interning to be able to handle nested allocations."] fn static_def_id (& self) -> Option < LocalDefId > ; }
    };
}

HasStaticRootDefId!()