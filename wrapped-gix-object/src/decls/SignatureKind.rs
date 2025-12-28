macro_rules! SignatureKind {
    () => {
        # [derive (Copy , Clone)] pub (crate) enum SignatureKind { Author , Committer , }
    };
}

SignatureKind!();