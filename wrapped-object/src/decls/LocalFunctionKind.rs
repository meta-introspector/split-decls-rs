macro_rules! LocalFunctionKind {
    () => {
        # [derive (Clone)] enum LocalFunctionKind { Unknown , Exported , }
    };
}

LocalFunctionKind!();