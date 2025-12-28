macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        # [doc = " The function fiat_25519_add adds two field elements."] impl Add for FieldElement { type Output = Self ; fn add (self , rhs : Self) -> Self :: Output { let mut ret = fiat_25519_tight_field_element ([0u64 ; 5]) ; let mut ret_add = fiat_25519_loose_field_element ([0u64 ; 5]) ; fiat_25519_add (& mut ret_add , & self . 0 , & rhs . 0) ; fiat_25519_carry (& mut ret , & ret_add) ; Self (ret) } }
    };
}

impl_369!();