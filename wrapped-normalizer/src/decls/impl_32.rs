macro_rules! deps {
    () => {
        CanonicalCombiningClassMapBorrowed!();
        CanonicalCombiningClass!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl CanonicalCombiningClassMapBorrowed < '_ > { # [doc = " Look up the canonical combining class for a scalar value."] # [doc = ""] # [doc = " The return value is a u8 representing the canonical combining class,"] # [doc = " you may enable the `\"icu_properties\"` feature if you would like to use a typed"] # [doc = " `CanonicalCombiningClass`."] # [inline (always)] pub fn get_u8 (& self , c : char) -> u8 { self . get32_u8 (u32 :: from (c)) } # [doc = " Look up the canonical combining class for a scalar value"] # [doc = " represented as `u32`. If the argument is outside the scalar"] # [doc = " value range, `Not_Reordered` is returned."] # [doc = ""] # [doc = " The return value is a u8 representing the canonical combining class,"] # [doc = " you may enable the `\"icu_properties\"` feature if you would like to use a typed"] # [doc = " `CanonicalCombiningClass`."] pub fn get32_u8 (& self , c : u32) -> u8 { let trie_value = self . decompositions . trie . get32 (c) ; if trie_value_has_ccc (trie_value) { trie_value as u8 } else { ccc ! (NotReordered , 0) . to_icu4c_value () } } # [doc = " Look up the canonical combining class for a scalar value"] # [doc = ""] # [doc = " ✨ *Enabled with the `icu_properties` Cargo feature.*"] # [inline (always)] # [cfg (feature = "icu_properties")] pub fn get (& self , c : char) -> CanonicalCombiningClass { CanonicalCombiningClass :: from_icu4c_value (self . get_u8 (c)) } # [doc = " Look up the canonical combining class for a scalar value"] # [doc = " represented as `u32`. If the argument is outside the scalar"] # [doc = " value range, `CanonicalCombiningClass::NotReordered` is returned."] # [doc = ""] # [doc = " ✨ *Enabled with the `icu_properties` Cargo feature.*"] # [cfg (feature = "icu_properties")] pub fn get32 (& self , c : u32) -> CanonicalCombiningClass { CanonicalCombiningClass :: from_icu4c_value (self . get32_u8 (c)) } }
    };
}

impl_32!()