macro_rules! deps {
    () => {
        Baked!();
        BinaryProperty!();
        PropertyCodePointSet!();
    };
}

macro_rules! make_binary_property {
    () => {
        deps!();
        macro_rules ! make_binary_property { (name : $ name : literal ; short_name : $ short_name : literal ; ident : $ ident : ident ; data_marker : $ data_marker : ty ; singleton : $ singleton : ident ; $ (# [$ doc : meta]) +) => { $ (# [$ doc]) + # [derive (Debug)] # [non_exhaustive] pub struct $ ident ; impl crate :: private :: Sealed for $ ident { } impl BinaryProperty for $ ident { type DataMarker = $ data_marker ; # [cfg (feature = "compiled_data")] const SINGLETON : &'static crate :: provider :: PropertyCodePointSet <'static > = & crate :: provider :: Baked ::$ singleton ; const NAME : &'static [u8] = $ name . as_bytes () ; const SHORT_NAME : &'static [u8] = $ short_name . as_bytes () ; } } ; }
    };
}

make_binary_property!();