macro_rules! did_you_mean {
    () => {
        # [cfg (not (feature = "suggestions"))] pub (crate) fn did_you_mean < T , I > (_ : & str , _ : I) -> Vec < String > where T : AsRef < str > , I : IntoIterator < Item = T > , { Vec :: new () }
    };
}

did_you_mean!();