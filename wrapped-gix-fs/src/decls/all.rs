macro_rules! deps {
    () => {
        Iter!();
        Retries!();
    };
}

macro_rules! all {
    () => {
        deps!();
        # [doc = " Create all directories leading to `dir` including `dir` itself with the specified amount of `retries`."] # [doc = " Returns the input `dir` on success that make it useful in expressions."] pub fn all (dir : & Path , retries : Retries) -> std :: io :: Result < & Path > { for res in Iter :: new_with_retries (dir , retries) { match res { Err (Error :: Permanent { err , .. }) => return Err (err) , Err (Error :: Intermediate { .. }) | Ok (_) => continue , } } Ok (dir) }
    };
}

all!();