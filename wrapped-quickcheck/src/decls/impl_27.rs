macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T : Arbitrary > Arbitrary for LinkedList < T > { fn arbitrary (g : & mut Gen) -> LinkedList < T > { let vec : Vec < T > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = LinkedList < T > > > { let vec : Vec < T > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < LinkedList < T > > ()) ,) } }
    };
}

impl_27!()