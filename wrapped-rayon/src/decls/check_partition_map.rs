macro_rules! check_partition_map {
    () => {
        # [test] fn check_partition_map () { let input = "a b c 1 2 3 x y z" ; let (a , b) : (Vec < _ > , String) = input . par_split_whitespace () . partition_map (| s | match s . parse :: < i32 > () { Ok (n) => Either :: Left (n) , Err (_) => Either :: Right (s) , }) ; assert_eq ! (a , vec ! [1 , 2 , 3]) ; assert_eq ! (b , "abcxyz") ; }
    };
}

check_partition_map!();