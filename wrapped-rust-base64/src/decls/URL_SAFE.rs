macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! URL_SAFE {
    () => {
        deps!();
        # [doc = " The URL-safe alphabet (with `-` and `_`) specified in [RFC 4648][]."] # [doc = ""] # [doc = " [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-5"] pub const URL_SAFE : Alphabet = Alphabet :: from_str_unchecked ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_" ,) ;
    };
}

URL_SAFE!()