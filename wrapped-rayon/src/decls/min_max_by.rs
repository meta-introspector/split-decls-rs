macro_rules! min_max_by {
    () => {
        # [test] fn min_max_by () { let rng = seeded_rng () ; let r : Vec < i32 > = rng . sample_iter (& StandardUniform) . take (512) . collect () ; let a : Vec < (i32 , u16) > = r . iter () . chain (& r) . cloned () . zip (0 ..) . collect () ; for i in 0 ..= a . len () { let slice = & a [.. i] ; assert_eq ! (slice . par_iter () . min_by (| x , y | x . 0 . cmp (& y . 0)) , slice . iter () . min_by (| x , y | x . 0 . cmp (& y . 0))) ; assert_eq ! (slice . par_iter () . max_by (| x , y | x . 0 . cmp (& y . 0)) , slice . iter () . max_by (| x , y | x . 0 . cmp (& y . 0))) ; } }
    };
}

min_max_by!();