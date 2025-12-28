macro_rules! CopiedFolder {
    () => {
        struct CopiedFolder < F > { base : F , }
    };
}

CopiedFolder!();