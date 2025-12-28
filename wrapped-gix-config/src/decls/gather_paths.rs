macro_rules! deps {
    () => {
        SectionId!();
        Section!();
    };
}

macro_rules! gather_paths {
    () => {
        deps!();
        fn gather_paths (section : & file :: Section < '_ > , id : SectionId) -> Vec < (SectionId , crate :: Path < 'static >) > { section . body . values ("path") . into_iter () . map (| path | (id , crate :: Path :: from (Cow :: Owned (path . into_owned ())))) . collect () }
    };
}

gather_paths!();