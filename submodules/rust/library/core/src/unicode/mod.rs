mkuse!{# [rustfmt :: skip] pub use unicode_data :: case_ignorable :: lookup as Case_Ignorable ;}
mkuse!{pub use unicode_data :: cased :: lookup as Cased ;}
mkuse!{pub use unicode_data :: conversions ;}
mkuse!{# [rustfmt :: skip] pub (crate) use unicode_data :: alphabetic :: lookup as Alphabetic ;}
mkuse!{pub (crate) use unicode_data :: grapheme_extend :: lookup as Grapheme_Extend ;}
mkuse!{pub (crate) use unicode_data :: lowercase :: lookup as Lowercase ;}
mkuse!{pub (crate) use unicode_data :: n :: lookup as N ;}
mkuse!{pub (crate) use unicode_data :: uppercase :: lookup as Uppercase ;}
mkuse!{pub (crate) use unicode_data :: white_space :: lookup as White_Space ;}
mkmod!{printable, { 
                getname!(printable);
                getsrc!(printable);
                getpath!(printable);
                get_deps!(printable);
                get_crates!(printable);
                mkinclude!(printable);
                 
            }}
mkmod!{unicode_data, { 
                getname!(unicode_data);
                getsrc!(unicode_data);
                getpath!(unicode_data);
                get_deps!(unicode_data);
                get_crates!(unicode_data);
                mkinclude!(unicode_data);
                 
            }}
mkitem!{# [doc = " The version of [Unicode](https://www.unicode.org/) that the Unicode parts of"] # [doc = " `char` and `str` methods are based on."] # [doc = ""] # [doc = " New versions of Unicode are released regularly and subsequently all methods"] # [doc = " in the standard library depending on Unicode are updated. Therefore the"] # [doc = " behavior of some `char` and `str` methods and the value of this constant"] # [doc = " changes over time. This is *not* considered to be a breaking change."] # [doc = ""] # [doc = " The version numbering scheme is explained in"] # [doc = " [Unicode 11.0 or later, Section 3.1 Versions of the Unicode Standard](https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf#page=4)."] pub const UNICODE_VERSION : (u8 , u8 , u8) = unicode_data :: UNICODE_VERSION ;}