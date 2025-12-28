macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! STANDARD {
    () => {
        deps!();
        # [doc = " The standard alphabet (with `+` and `/`) specified in [RFC 4648][]."] # [doc = ""] # [doc = " [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-4"] pub const STANDARD : Alphabet = Alphabet :: from_str_unchecked ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" ,) ;
    };
}

STANDARD!();