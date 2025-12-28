macro_rules! ResolvedPath {
    () => {
        pub (crate) struct ResolvedPath { pub (crate) resolution : hir :: PathResolution , # [doc = " The depth of the ast::Path that was resolved within the pattern."] pub (crate) depth : u32 , }
    };
}

ResolvedPath!()