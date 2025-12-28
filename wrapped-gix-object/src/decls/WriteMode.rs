macro_rules! WriteMode {
    () => {
        enum WriteMode { Normal , # [doc = " Perform less cleanup to assure parent-editor still stays intact"] FromCursor , }
    };
}

WriteMode!();