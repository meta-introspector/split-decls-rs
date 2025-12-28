macro_rules! deps {
    () => {
        Bits!();
        Flag!();
        Flags!();
    };
}

macro_rules! __impl_public_bitflags_consts {
    () => {
        deps!();
        # [doc = " Implement constants on the public (user-facing) bitflags type."] # [macro_export] # [doc (hidden)] macro_rules ! __impl_public_bitflags_consts { ($ (# [$ outer : meta]) * $ PublicBitFlags : ident : $ T : ty { $ ($ (# [$ inner : ident $ ($ args : tt) *]) * const $ Flag : tt = $ value : expr ;) * }) => { $ (# [$ outer]) * impl $ PublicBitFlags { $ ($ crate :: __bitflags_flag ! ({ name : $ Flag , named : { $ (# [$ inner $ ($ args) *]) * # [allow (deprecated , non_upper_case_globals ,)] pub const $ Flag : Self = Self :: from_bits_retain ($ value) ; } , unnamed : { } , }) ;) * } $ (# [$ outer]) * impl $ crate :: Flags for $ PublicBitFlags { const FLAGS : &'static [$ crate :: Flag <$ PublicBitFlags >] = & [$ ($ crate :: __bitflags_flag ! ({ name : $ Flag , named : { $ crate :: __bitflags_expr_safe_attrs ! ($ (# [$ inner $ ($ args) *]) * { # [allow (deprecated , non_upper_case_globals ,)] $ crate :: Flag :: new ($ crate :: __private :: core :: stringify ! ($ Flag) , $ PublicBitFlags ::$ Flag) }) } , unnamed : { $ crate :: __bitflags_expr_safe_attrs ! ($ (# [$ inner $ ($ args) *]) * { # [allow (deprecated , non_upper_case_globals ,)] $ crate :: Flag :: new ("" , $ PublicBitFlags :: from_bits_retain ($ value)) }) } , }) ,) *] ; type Bits = $ T ; fn bits (& self) -> $ T { $ PublicBitFlags :: bits (self) } fn from_bits_retain (bits : $ T) -> $ PublicBitFlags { $ PublicBitFlags :: from_bits_retain (bits) } } } ; }
    };
}

__impl_public_bitflags_consts!()