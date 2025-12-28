macro_rules! Ignore {
    () => {
        # [doc = " An implementation of the [`Pattern`] trait for ignore-patterns."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub struct Ignore { # [doc = " If `support_precious` is `true`, we will parse `$` prefixed entries as precious."] # [doc = " This is backward-incompatible as files that actually start with `$` like `$houdini`"] # [doc = " will then not be ignored anymore, instead it ignores `houdini`."] pub support_precious : bool , }
    };
}

Ignore!();