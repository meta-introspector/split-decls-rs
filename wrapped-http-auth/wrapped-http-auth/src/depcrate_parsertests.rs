// Generated macro for tests (module)
macro_rules! Depcrate_parsertests {
() => {
// Module: crate::parser
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { ChallengeRef , ParamValue } ; # [test] fn multi_challenge () { let input = r#"Newauth realm="apps", type=1, title="Login to \"apps\"", Basic realm="simple""# ; let challenges = crate :: parse_challenges (input) . unwrap () ; assert_eq ! (& challenges [..] , & [ChallengeRef { scheme : "Newauth" , params : vec ! [("realm" , ParamValue :: new (0 , "apps")) , ("type" , ParamValue :: new (0 , "1")) , ("title" , ParamValue :: new (2 , r#"Login to \"apps\""#)) ,] , } , ChallengeRef { scheme : "Basic" , params : vec ! [("realm" , ParamValue :: new (0 , "simple")) ,] , } ,]) ; } # [test] fn empty () { crate :: parse_challenges ("") . unwrap_err () ; crate :: parse_challenges (",") . unwrap_err () ; } }
};
}
