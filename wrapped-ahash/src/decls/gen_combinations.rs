macro_rules! gen_combinations {
    () => {
        fn gen_combinations (options : & [u32 ; 11] , depth : u32 , so_far : Vec < u32 > , combinations : & mut Vec < Vec < u32 > >) { if depth == 0 { return ; } for option in options { let mut next = so_far . clone () ; next . push (* option) ; combinations . push (next . clone ()) ; gen_combinations (options , depth - 1 , next , combinations) ; } }
    };
}

gen_combinations!();