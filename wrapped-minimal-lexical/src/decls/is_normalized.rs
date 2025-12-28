macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! is_normalized {
    () => {
        deps!();
        # [doc = " Get if the big integer is normalized."] # [inline] # [allow (clippy :: match_like_matches_macro)] pub fn is_normalized (x : & [Limb]) -> bool { match x . get (x . len () . wrapping_sub (1)) { Some (& 0) => false , _ => true , } }
    };
}

is_normalized!();