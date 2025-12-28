macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! sub_poly {
    () => {
        deps!();
        fn sub_poly (p1 : & [FieldElement ; 256] , p2 : & [FieldElement ; 256] , ret : & mut [FieldElement ; 256]) { for idx in 0 .. 256 { ret [idx] = p1 [idx] - p2 [idx] ; } }
    };
}

sub_poly!();