macro_rules! max_by_key {
    () => {
        # [test] fn max_by_key () { let it = convert (vec ! [0 , 3 , 1 , - 10] . into_iter () . map (Ok :: < i32 , i32 >)) ; assert_eq ! (it . clone () . max_by_key (|& i | Ok (- i)) , Ok (Some (- 10))) ; assert_eq ! (it . clone () . max_by_key (|& i | Err ::< i32 , _ > (i)) , Err (0)) ; assert_eq ! (it . max_by_key (|& i | if i > 0 { Err (i) } else { Ok (- i) }) , Err (3)) ; }
    };
}

max_by_key!()