macro_rules! deps {
    () => {
        Consumer!();
    };
}

macro_rules! Reducer {
    () => {
        deps!();
        # [doc = " The reducer is the final step of a `Consumer` -- after a consumer"] # [doc = " has been split into two parts, and each of those parts has been"] # [doc = " fully processed, we are left with two results. The reducer is then"] # [doc = " used to combine those two results into one. See [the `plumbing`"] # [doc = " README][r] for further details."] # [doc = ""] # [doc = " [r]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md"] pub trait Reducer < Result > { # [doc = " Reduce two final results into one; this is executed after a"] # [doc = " split."] fn reduce (self , left : Result , right : Result) -> Result ; }
    };
}

Reducer!()