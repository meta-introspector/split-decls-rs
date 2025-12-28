macro_rules! deps {
    () => {
        Action!();
        Result!();
        Outcome!();
        Error!();
    };
}

macro_rules! helper_outcome_to_result {
    () => {
        deps!();
        # [doc = " Convert the outcome of a helper invocation to a helper result, assuring that the identity is complete in the process."] # [allow (clippy :: result_large_err)] pub fn helper_outcome_to_result (outcome : Option < helper :: Outcome > , action : helper :: Action) -> Result { match (action , outcome) { (helper :: Action :: Get (ctx) , None) => Err (Error :: IdentityMissing { context : ctx . redacted () , }) , (helper :: Action :: Get (ctx) , Some (mut outcome)) => match outcome . consume_identity () { Some (identity) => Ok (Some (Outcome { identity , next : outcome . next , })) , None => Err (if outcome . quit { Error :: Quit } else { Error :: IdentityMissing { context : ctx . redacted () , } }) , } , (helper :: Action :: Store (_) | helper :: Action :: Erase (_) , _ignore) => Ok (None) , } }
    };
}

helper_outcome_to_result!()