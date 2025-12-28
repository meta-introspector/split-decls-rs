macro_rules! deps {
    () => {
        MatchError!();
        Input!();
    };
}

macro_rules! skip_splits_fwd {
    () => {
        deps!();
        # [cold] # [inline (never)] pub (crate) fn skip_splits_fwd < T , F > (input : & Input < '_ > , init_value : T , match_offset : usize , find : F ,) -> Result < Option < T > , MatchError > where F : FnMut (& Input < '_ >) -> Result < Option < (T , usize) > , MatchError > , { skip_splits (true , input , init_value , match_offset , find) }
    };
}

skip_splits_fwd!();