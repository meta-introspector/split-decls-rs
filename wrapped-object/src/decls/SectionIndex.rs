macro_rules! SectionIndex {
    () => {
        # [doc = " The index of an ELF section."] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct SectionIndex (pub u32) ;
    };
}

SectionIndex!()