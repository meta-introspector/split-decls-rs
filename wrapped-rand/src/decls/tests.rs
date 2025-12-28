macro_rules! deps {
    () => {
        Xoshiro256PlusPlus!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: Xoshiro256PlusPlus ; use rand_core :: { RngCore , SeedableRng } ; # [test] fn reference () { let mut rng = Xoshiro256PlusPlus :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,]) ; let expected = [41943041 , 58720359 , 3588806011781223 , 3591011842654386 , 9228616714210784205 , 9973669472204895162 , 14011001112246962877 , 12406186145184390807 , 15849039046786891736 , 10450023813501588000 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } # [test] fn stable_seed_from_u64_and_from_seed () { let mut rng = Xoshiro256PlusPlus :: seed_from_u64 (0) ; let mut rng_from_seed_0 = Xoshiro256PlusPlus :: from_seed ([0 ; 32]) ; let expected = [5987356902031041503 , 7051070477665621255 , 6633766593972829180 , 211316841551650330 , 9136120204379184874 , 379361710973160858 , 15813423377499357806 , 15596884590815070553 , 5439680534584881407 , 1369371744833522710 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; assert_eq ! (rng_from_seed_0 . next_u64 () , e) ; } } }
    };
}

tests!()