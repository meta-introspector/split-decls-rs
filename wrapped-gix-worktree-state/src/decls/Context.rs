macro_rules! Context {
    () => {
        pub struct Context < 'a , Find > { pub objects : & 'a mut Find , pub path_cache : & 'a mut Stack , pub filters : & 'a mut gix_filter :: Pipeline , pub buf : & 'a mut Vec < u8 > , }
    };
}

Context!()