macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! OnePassCache {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct OnePassCache (# [cfg (feature = "dfa-onepass")] Option < onepass :: Cache > , # [cfg (not (feature = "dfa-onepass"))] () ,) ;
    };
}

OnePassCache!()