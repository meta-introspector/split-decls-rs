macro_rules! MaxArrayLengthP1 {
    () => {
        type MaxArrayLengthP1 = typenum :: Shleft < typenum :: U1 , typenum :: Shleft < typenum :: U < { mem :: size_of :: < usize > () } > , typenum :: U3 > , > ;
    };
}

MaxArrayLengthP1!();