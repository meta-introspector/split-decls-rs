// Generated macro for challenges (function)
macro_rules! Depcratechallenges {
() => {
// Module: crate
// Provides: {"challenges"}
// Dependencies: {}
# [doc = " Appends the challenges described by `value` into `challenges`."] # [doc = ""] # [doc = " This can be used to parse `Proxy-Authenticate` and/or `WWW-Authenticate` header values."] # [doc = ""] # [doc = " ```text"] # [doc = "   Proxy-Authenticate = *( \",\" OWS ) challenge *( OWS \",\" [ OWS"] # [doc = "    challenge ] )"] # [doc = ""] # [doc = "   WWW-Authenticate = *( \",\" OWS ) challenge *( OWS \",\" [ OWS challenge"] # [doc = "    ] )"] # [doc = " ```"] pub fn challenges (input : & str) -> nom :: IResult < & str , Vec < ChallengeRef < '_ > > > { all_consuming (list1_relaxed (challenge)) (input) }
};
}
