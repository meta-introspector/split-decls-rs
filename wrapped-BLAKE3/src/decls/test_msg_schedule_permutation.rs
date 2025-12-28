macro_rules! test_msg_schedule_permutation {
    () => {
        # [test] fn test_msg_schedule_permutation () { let permutation = [2 , 6 , 3 , 10 , 7 , 0 , 4 , 13 , 1 , 11 , 12 , 5 , 9 , 14 , 15 , 8] ; let mut generated = [[0 ; 16] ; 7] ; generated [0] = [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15] ; for round in 1 .. 7 { for i in 0 .. 16 { generated [round] [i] = generated [round - 1] [permutation [i]] ; } } assert_eq ! (generated , crate :: MSG_SCHEDULE) ; }
    };
}

test_msg_schedule_permutation!()