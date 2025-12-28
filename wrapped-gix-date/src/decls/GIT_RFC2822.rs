macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! GIT_RFC2822 {
    () => {
        deps!();
        # [doc = " E.g. `Thu, 8 Aug 2022 12:45:06 +0800`. This is output by `git log --pretty=%aD`."] pub const GIT_RFC2822 : CustomFormat = CustomFormat ("%a, %-d %b %Y %H:%M:%S %z") ;
    };
}

GIT_RFC2822!();