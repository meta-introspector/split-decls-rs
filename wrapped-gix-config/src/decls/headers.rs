macro_rules! deps {
    () => {
        SectionId!();
        Section!();
        Header!();
    };
}

macro_rules! headers {
    () => {
        deps!();
        fn headers < 'a > (sections : & HashMap < SectionId , Section < 'a > >) -> HashMap < SectionId , section :: Header < 'a > > { sections . iter () . map (| (k , v) | (* k , v . header . clone ())) . collect () }
    };
}

headers!()