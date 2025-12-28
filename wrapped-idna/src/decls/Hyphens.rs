macro_rules! Hyphens {
    () => {
        # [doc = " The _CheckHyphens_ mode."] # [derive (PartialEq , Eq , Copy , Clone)] # [non_exhaustive] pub enum Hyphens { # [doc = " _CheckHyphens=false_: Do not place positional restrictions on hyphens."] # [doc = ""] # [doc = " This mode is used by the WHATWG URL Standard for normal User Agent processing"] # [doc = " (i.e. not conformance checking)."] Allow , # [doc = " Prohibit hyphens in the first and last position in the label but allow in"] # [doc = " the third and fourth position."] # [doc = ""] # [doc = " Note that this mode rejects real-world names, including some GitHub user pages."] CheckFirstLast , # [doc = " _CheckHyphens=true_: Prohibit hyphens in the first, third, fourth,"] # [doc = " and last position in the label."] # [doc = ""] # [doc = " Note that this mode rejects real-world names, including YouTube CDN nodes"] # [doc = " and some GitHub user pages."] Check , }
    };
}

Hyphens!()