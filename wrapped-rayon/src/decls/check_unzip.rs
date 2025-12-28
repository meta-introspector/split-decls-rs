macro_rules! check_unzip {
    () => {
        # [test] fn check_unzip () { let (a , b) : (Vec < _ > , HashSet < _ >) = (0 .. 1024) . into_par_iter () . map (| i | i * i) . enumerate () . unzip () ; let (c , d) : (Vec < _ > , HashSet < _ >) = (0 .. 1024) . map (| i | i * i) . enumerate () . unzip () ; assert_eq ! (a , c) ; assert_eq ! (b , d) ; let (a , b) : (HashSet < _ > , Vec < _ >) = (0 .. 1024) . into_par_iter () . map (| i | i * i) . enumerate () . unzip () ; let (c , d) : (HashSet < _ > , Vec < _ >) = (0 .. 1024) . map (| i | i * i) . enumerate () . unzip () ; assert_eq ! (a , c) ; assert_eq ! (b , d) ; let (a , b) : (Vec < _ > , Vec < _ >) = (0 .. 1024) . into_par_iter () . map (| i | i * i) . enumerate () . unzip () ; let (c , d) : (Vec < _ > , Vec < _ >) = (0 .. 1024) . map (| i | i * i) . enumerate () . unzip () ; assert_eq ! (a , c) ; assert_eq ! (b , d) ; let (a , b) : (Vec < _ > , Vec < _ >) = (0 .. 1024) . into_par_iter () . filter_map (| i | Some ((i , i * i))) . unzip () ; let (c , d) : (Vec < _ > , Vec < _ >) = (0 .. 1024) . map (| i | (i , i * i)) . unzip () ; assert_eq ! (a , c) ; assert_eq ! (b , d) ; }
    };
}

check_unzip!()