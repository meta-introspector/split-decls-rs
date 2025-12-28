macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! IMAP_MUTF7 {
    () => {
        deps!();
        # [doc = " The alphabet used in IMAP-modified UTF-7 (with `+` and `,`)."] # [doc = ""] # [doc = " See [RFC 3501](https://tools.ietf.org/html/rfc3501#section-5.1.3)"] pub const IMAP_MUTF7 : Alphabet = Alphabet :: from_str_unchecked ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+," ,) ;
    };
}

IMAP_MUTF7!();