macro_rules! deps {
    () => {
        Options!();
        Error!();
    };
}

macro_rules! imp {
    () => {
        deps!();
        # [cfg (not (unix))] mod imp { use crate :: { Error , Options } ; pub (crate) fn ask (_prompt : & str , _opts : & Options < '_ >) -> Result < String , Error > { Err (Error :: UnsupportedPlatform) } }
    };
}

imp!()