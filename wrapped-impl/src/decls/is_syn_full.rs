macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! is_syn_full {
    () => {
        deps!();
        fn is_syn_full () -> bool { let test = quote ! ({ trait Trait { } }) ; match syn :: parse2 (test) { Ok (Expr :: Verbatim (_)) | Err (_) => false , Ok (Expr :: Block (_)) => true , Ok (_) => unreachable ! () , } }
    };
}

is_syn_full!()