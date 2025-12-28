macro_rules! check_unzip_into_vecs {
    () => {
        # [test] fn check_unzip_into_vecs () { let mut a = vec ! [] ; let mut b = vec ! [] ; (0 .. 1024) . into_par_iter () . map (| i | i * i) . enumerate () . unzip_into_vecs (& mut a , & mut b) ; let (c , d) : (Vec < _ > , Vec < _ >) = (0 .. 1024) . map (| i | i * i) . enumerate () . unzip () ; assert_eq ! (a , c) ; assert_eq ! (b , d) ; }
    };
}

check_unzip_into_vecs!();