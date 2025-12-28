macro_rules! deps {
    () => {
        ChallengeRef!();
    };
}

macro_rules! ParamValue {
    () => {
        deps!();
        # [doc = " Parsed challenge parameter value used within [`ChallengeRef`]."] # [derive (Copy , Clone , Eq , PartialEq)] pub struct ParamValue < 'i > { # [doc = " The number of backslash escapes in a quoted-text parameter; 0 for a plain token."] escapes : usize , # [doc = " The escaped string, which must be pure ASCII (no bytes >= 128) and be"] # [doc = " consistent with `escapes`."] escaped : & 'i str , }
    };
}

ParamValue!()