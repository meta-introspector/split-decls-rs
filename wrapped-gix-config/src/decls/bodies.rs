macro_rules! deps {
    () => {
        Body!();
        SectionId!();
        Section!();
    };
}

macro_rules! bodies {
    () => {
        deps!();
        fn bodies < 'a > (sections : & HashMap < SectionId , Section < 'a > >) -> HashMap < SectionId , file :: section :: Body < 'a > > { sections . iter () . map (| (k , v) | (* k , v . body . clone ())) . collect () }
    };
}

bodies!();