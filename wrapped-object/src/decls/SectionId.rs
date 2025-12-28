macro_rules! deps {
    () => {
        Sections!();
    };
}

macro_rules! SectionId {
    () => {
        deps!();
        # [doc = " An ID for referring to a section in [`Sections`]."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct SectionId (usize) ;
    };
}

SectionId!()