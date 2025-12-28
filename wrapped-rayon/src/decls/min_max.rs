macro_rules! min_max {
    () => {
        # [test] fn min_max () { let rng = seeded_rng () ; let a : Vec < i32 > = rng . sample_iter (& StandardUniform) . take (1024) . collect () ; for i in 0 ..= a . len () { let slice = & a [.. i] ; assert_eq ! (slice . par_iter () . min () , slice . iter () . min ()) ; assert_eq ! (slice . par_iter () . max () , slice . iter () . max ()) ; } }
    };
}

min_max!();