// Generated macro for challenge (function)
macro_rules! Depcratechallenge {
() => {
// Module: crate
// Provides: {"challenge"}
// Dependencies: {}
# [doc = " Parses a challenge as in [RFC 7235]."] # [doc = ""] # [doc = " Section 2.1 defines this rule as follows:"] # [doc = " ```text"] # [doc = " auth-scheme = token"] # [doc = " challenge   = auth-scheme [ 1*SP ( token68 / #auth-param ) ]"] # [doc = " ```"] # [doc = ""] # [doc = " Although in practice this is ambiguous when placed into a `1#challenge`,"] # [doc = " which we resolve by using `list0_relaxed_inner` rather than `list0_relaxed`."] fn challenge (input : & str) -> nom :: IResult < & str , ChallengeRef < '_ > > { trace ! ("challenge attempt on {:?}" , input) ; map (tuple ((token , opt (preceded (char (' ') , list0_relaxed_inner (auth_param))) ,)) , | (scheme , opt_params) | ChallengeRef { scheme , params : opt_params . unwrap_or_default () , } ,) (input) }
};
}
