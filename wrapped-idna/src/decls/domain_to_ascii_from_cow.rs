macro_rules! deps {
    () => {
        DnsLength!();
        Hyphens!();
        AsciiDenyList!();
        Errors!();
        Uts46!();
    };
}

macro_rules! domain_to_ascii_from_cow {
    () => {
        deps!();
        # [doc = " The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm;"] # [doc = " version accepting and returning a `Cow`."] # [doc = ""] # [doc = " Most applications should be using this function or `domain_to_ascii_cow` rather"] # [doc = " than the sibling functions, and most applications should pass [`AsciiDenyList::URL`] as"] # [doc = " the second argument. Passing [`AsciiDenyList::URL`] as the second argument makes this function also"] # [doc = " perform the [forbidden domain code point](https://url.spec.whatwg.org/#forbidden-domain-code-point)"] # [doc = " check in addition to the [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii)"] # [doc = " algorithm."] # [doc = ""] # [doc = " Return the ASCII representation a domain name,"] # [doc = " normalizing characters (upper-case to lower-case and other kinds of equivalence)"] # [doc = " and using Punycode as necessary."] # [doc = ""] # [doc = " This process may fail."] pub fn domain_to_ascii_from_cow (domain : Cow < '_ , [u8] > , ascii_deny_list : AsciiDenyList ,) -> Result < Cow < '_ , str > , Errors > { Uts46 :: new () . to_ascii_from_cow (domain , ascii_deny_list , uts46 :: Hyphens :: Allow , uts46 :: DnsLength :: Ignore ,) }
    };
}

domain_to_ascii_from_cow!()