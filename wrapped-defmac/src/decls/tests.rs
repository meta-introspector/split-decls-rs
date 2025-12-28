macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { # [test] fn it_works () { let value = "xyz" ; defmac ! (none => value) ; assert_eq ! (none ! () , "xyz") ; defmac ! (one x => x) ; assert_eq ! (one ! (2) , 2) ; defmac ! (two x , y => x + y) ; assert_eq ! (two ! (1. , 2.) , 3.) ; defmac ! (three x , y , z => (x , y , z)) ; assert_eq ! (three ! (1 , (2 , 3) , (4 , 5 , 6)) , (1 , (2 , 3) , (4 , 5 , 6))) ; defmac ! (four w , x , y , z => (w + x , z , y)) ; assert_eq ! (four ! (3 , 4 , "a" , "b") , (7 , "b" , "a")) ; defmac ! (many a , b , c , d , e , f , g , h , i , j , k => (a , b + c , d + e + f , g + h + i + j + k)) ; assert_eq ! (many ! (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11) , (1 , 5 , 15 , 45)) ; } # [test] fn eval_order () { use std :: cell :: Cell ; let v = Cell :: new (0) ; let f = | | { let n = v . get () ; v . set (n + 1) ; n } ; defmac ! (two x , y => (x , y)) ; let result = two ! (f () , f ()) ; assert_eq ! (result , (0 , 1)) ; assert_eq ! (f () , 2) ; } defmac ! { triple value => [value ; 3] } # [test] fn module_macro () { assert_eq ! (triple ! (3) , [3 , 3 , 3]) ; } }
    };
}

tests!();