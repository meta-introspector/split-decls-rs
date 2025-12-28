macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl < 'data > PropertyCodePointSet < 'data > { # [inline] pub (crate) fn contains (& self , ch : char) -> bool { match * self { Self :: InversionList (ref l) => l . contains (ch) , } } # [inline] pub (crate) fn contains32 (& self , ch : u32) -> bool { match * self { Self :: InversionList (ref l) => l . contains32 (ch) , } } # [inline] pub (crate) fn iter_ranges (& self) -> impl Iterator < Item = RangeInclusive < u32 > > + '_ { match * self { Self :: InversionList (ref l) => l . iter_ranges () , } } # [inline] pub (crate) fn iter_ranges_complemented (& self ,) -> impl Iterator < Item = RangeInclusive < u32 > > + '_ { match * self { Self :: InversionList (ref l) => l . iter_ranges_complemented () , } } # [inline] pub (crate) fn from_code_point_inversion_list (l : CodePointInversionList < 'static >) -> Self { Self :: InversionList (l) } # [inline] pub (crate) fn as_code_point_inversion_list (& '_ self ,) -> Option < & '_ CodePointInversionList < 'data > > { match * self { Self :: InversionList (ref l) => Some (l) , } } # [inline] pub (crate) fn to_code_point_inversion_list (& self) -> CodePointInversionList < '_ > { match * self { Self :: InversionList (ref t) => ZeroFrom :: zero_from (t) , } } }
    };
}

impl_367!();