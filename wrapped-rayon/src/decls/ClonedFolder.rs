macro_rules! ClonedFolder {
    () => {
        struct ClonedFolder < F > { base : F , }
    };
}

ClonedFolder!()