macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! FollowRedirects {
    () => {
        deps!();
        # [doc = " The `http.followRedirects` key."] pub type FollowRedirects = keys :: Any < validate :: FollowRedirects > ;
    };
}

FollowRedirects!();