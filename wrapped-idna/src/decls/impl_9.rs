macro_rules! deps {
    () => {
        Config!();
        AsciiDenyList!();
        Errors!();
        Hyphens!();
        Idna!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Config { # [doc = " Whether to enforce STD3 or WHATWG URL Standard ASCII deny list."] # [doc = ""] # [doc = " `true` for STD3, `false` for no deny list."] # [doc = ""] # [doc = " Note that `true` rejects pseudo-hosts used by various TXT record-based protocols."] # [inline] pub fn use_std3_ascii_rules (mut self , value : bool) -> Self { self . use_std3_ascii_rules = value ; self } # [doc = " Whether to enable (deprecated) transitional processing."] # [doc = ""] # [doc = " Note that Firefox, Safari, and Chrome do not use transitional"] # [doc = " processing."] # [inline] pub fn transitional_processing (mut self , value : bool) -> Self { self . transitional_processing = value ; self } # [doc = " Whether the _VerifyDNSLength_ operation should be performed"] # [doc = " by `to_ascii`."] # [doc = ""] # [doc = " For compatibility with previous behavior, even when set to `true`,"] # [doc = " the trailing root label dot is allowed contrary to the spec."] # [inline] pub fn verify_dns_length (mut self , value : bool) -> Self { self . verify_dns_length = value ; self } # [doc = " Whether to enforce STD3 rules for hyphen placement."] # [doc = ""] # [doc = " `true` to deny hyphens in the first and last positions."] # [doc = " `false` to not enforce hyphen placement."] # [doc = ""] # [doc = " Note that for backward compatibility this is not the same as"] # [doc = " UTS 46 _CheckHyphens_, which also disallows hyphens in the"] # [doc = " third and fourth positions."] # [doc = ""] # [doc = " Note that `true` rejects real-world names, including some GitHub user pages."] # [inline] pub fn check_hyphens (mut self , value : bool) -> Self { self . check_hyphens = value ; self } # [doc = " Obsolete method retained to ease migration. The argument must be `false`."] # [doc = ""] # [doc = " Panics"] # [doc = ""] # [doc = " If the argument is `true`."] # [inline] # [allow (unused_mut)] pub fn use_idna_2008_rules (mut self , value : bool) -> Self { assert ! (! value , "IDNA 2008 rules are no longer supported") ; self } # [doc = " Compute the deny list"] fn deny_list (& self) -> AsciiDenyList { if self . use_std3_ascii_rules { AsciiDenyList :: STD3 } else { AsciiDenyList :: EMPTY } } # [doc = " Compute the hyphen mode"] fn hyphens (& self) -> Hyphens { if self . check_hyphens { Hyphens :: CheckFirstLast } else { Hyphens :: Allow } } # [doc = " [UTS 46 ToASCII](http://www.unicode.org/reports/tr46/#ToASCII)"] pub fn to_ascii (self , domain : & str) -> Result < String , Errors > { let mut result = String :: with_capacity (domain . len ()) ; let mut codec = Idna :: new (self) ; codec . to_ascii (domain , & mut result) . map (| () | result) } # [doc = " [UTS 46 ToUnicode](http://www.unicode.org/reports/tr46/#ToUnicode)"] pub fn to_unicode (self , domain : & str) -> (String , Result < () , Errors >) { let mut codec = Idna :: new (self) ; let mut out = String :: with_capacity (domain . len ()) ; let result = codec . to_unicode (domain , & mut out) ; (out , result) } }
    };
}

impl_9!();