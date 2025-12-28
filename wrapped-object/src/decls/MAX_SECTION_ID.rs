macro_rules! deps {
    () => {
        SectionId!();
    };
}

macro_rules! MAX_SECTION_ID {
    () => {
        deps!();
        const MAX_SECTION_ID : usize = SectionId :: Tag as usize ;
    };
}

MAX_SECTION_ID!()