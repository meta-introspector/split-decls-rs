macro_rules! FrontmatterAllowed {
    () => {
        pub enum FrontmatterAllowed { Yes , No , }
    };
}

FrontmatterAllowed!();