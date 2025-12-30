// Generated macro for generate_rand_max_num_actions (function)
macro_rules! Depcrategenerate_rand_max_num_actions {
() => {
// Module: crate
// Provides: {"generate_rand_max_num_actions"}
// Dependencies: {}
# [doc = " Using a Skewed Normal distribution, we generate a random number which indicates the maximum"] # [doc = " number of actions we'll run for a [`Scenario`]"] # [doc = ""] # [doc = " Note: through testing we seem"] fn generate_rand_max_num_actions (seed : u64) -> usize { let mut rng = SmallRng :: seed_from_u64 (seed) ; let poi = SkewNormal :: new (6.0 , 6.0 , 3.0) . expect ("invalid SkewNormal") ; let smp = poi . sample (& mut rng) ; let num_act = smp * 100f64 ; let num_act = num_act as u64 % 5000 ; num_act as usize }
};
}
