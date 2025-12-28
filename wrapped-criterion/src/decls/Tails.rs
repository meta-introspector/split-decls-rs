macro_rules! Tails {
    () => {
        # [doc = " Number of tails for significance testing"] pub enum Tails { # [doc = " One tailed test"] One , # [doc = " Two tailed test"] Two , }
    };
}

Tails!();