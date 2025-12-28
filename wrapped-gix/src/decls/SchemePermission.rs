macro_rules! deps {
    () => {
        Allow!();
        Clone!();
    };
}

macro_rules! SchemePermission {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (crate) struct SchemePermission { # [doc = " `None`, env-var is unset or wasn't queried, otherwise true if `GIT_PROTOCOL_FROM_USER` is `1`."] user_allowed : Option < bool > , # [doc = " The general allow value from `protocol.allow`."] allow : Option < Allow > , # [doc = " Per scheme allow information"] allow_per_scheme : BTreeMap < gix_url :: Scheme , Allow > , }
    };
}

SchemePermission!();