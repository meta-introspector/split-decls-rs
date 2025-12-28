macro_rules! deps {
    () => {
        Url!();
    };
}

macro_rules! UrlKind {
    () => {
        deps!();
        # [doc = ""] # [derive (Debug , Clone , Copy)] pub enum UrlKind { # [doc = ""] Url , # [doc = ""] Scp , # [doc = ""] Local , }
    };
}

UrlKind!();