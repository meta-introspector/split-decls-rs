macro_rules! AnnotationLocation {
    () => {
        pub enum AnnotationLocation { AboveName , AboveWholeItem , }
    };
}

AnnotationLocation!();