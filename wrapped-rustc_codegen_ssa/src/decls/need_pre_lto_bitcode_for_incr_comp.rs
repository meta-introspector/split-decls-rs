macro_rules! need_pre_lto_bitcode_for_incr_comp {
    () => {
        fn need_pre_lto_bitcode_for_incr_comp (sess : & Session) -> bool { if sess . opts . incremental . is_none () { return false ; } match sess . lto () { Lto :: No => false , Lto :: Fat | Lto :: Thin | Lto :: ThinLocal => true , } }
    };
}

need_pre_lto_bitcode_for_incr_comp!()