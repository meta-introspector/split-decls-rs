macro_rules! deps {
    () => {
        FmtArguments!();
    };
}

macro_rules! explicit_named_args {
    () => {
        deps!();
        # [allow (clippy :: unnecessary_wraps)] fn explicit_named_args (input : ParseStream) -> Result < FmtArguments > { let ahead = input . fork () ; if let Ok (set) = try_explicit_named_args (& ahead) { input . advance_to (& ahead) ; return Ok (set) ; } let ahead = input . fork () ; if let Ok (set) = fallback_explicit_named_args (& ahead) { input . advance_to (& ahead) ; return Ok (set) ; } input . parse :: < TokenStream > () . unwrap () ; Ok (FmtArguments { named : BTreeSet :: new () , first_unnamed : None , }) }
    };
}

explicit_named_args!()