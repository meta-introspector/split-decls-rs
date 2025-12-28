macro_rules! migration_guide {
    () => {
        # [cfg (doc)] pub mod migration_guide { # ! [doc = include_str ! ("../docs/migration_guide.md")] }
    };
}

migration_guide!();