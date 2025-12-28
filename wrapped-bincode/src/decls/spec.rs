macro_rules! spec {
    () => {
        # [cfg (all (feature = "alloc" , feature = "derive" , doc))] pub mod spec { # ! [doc = include_str ! ("../docs/spec.md")] }
    };
}

spec!();