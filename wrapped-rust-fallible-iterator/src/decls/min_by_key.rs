macro_rules! min_by_key {
    () => {
        # [test] fn min_by_key () { let it = convert (vec ! [0 , 3 , 1 , - 10] . into_iter () . map (Ok :: < i32 , i32 >)) ; assert_eq ! (it . clone () . min_by_key (|& i | Ok (- i)) , Ok (Some (3))) ; assert_eq ! (it . clone () . min_by_key (|& i | Err ::< i32 , _ > (i)) , Err (0)) ; assert_eq ! (it . min_by_key (|& i | if i > 0 { Err (i) } else { Ok (- i) }) , Err (3)) ; }
    };
}

min_by_key!();