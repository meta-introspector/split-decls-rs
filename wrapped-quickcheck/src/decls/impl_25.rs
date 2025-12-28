macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T : Arbitrary + Ord > Arbitrary for BinaryHeap < T > { fn arbitrary (g : & mut Gen) -> BinaryHeap < T > { let vec : Vec < T > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = BinaryHeap < T > > > { let vec : Vec < T > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < BinaryHeap < T > > ()) ,) } }
    };
}

impl_25!()