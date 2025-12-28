macro_rules! deprecated_mach {
    () => {
        macro_rules ! deprecated_mach { (pub const $ id : ident : $ ty : ty = $ expr : expr ;) => { # [deprecated (since = "0.2.55" , note = "Use the `mach2` crate instead" ,)] # [allow (deprecated)] pub const $ id : $ ty = $ expr ; } ; ($ (pub const $ id : ident : $ ty : ty = $ expr : expr ;) *) => { $ (deprecated_mach ! (pub const $ id : $ ty = $ expr ;) ;) * } ; (pub type $ id : ident = $ ty : ty ;) => { # [deprecated (since = "0.2.55" , note = "Use the `mach2` crate instead" ,)] # [allow (deprecated)] pub type $ id = $ ty ; } ; ($ (pub type $ id : ident = $ ty : ty ;) *) => { $ (deprecated_mach ! (pub type $ id = $ ty ;) ;) * } }
    };
}

deprecated_mach!()