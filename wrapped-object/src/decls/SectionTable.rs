macro_rules! deps {
    () => {
        FileHeader!();
        SectionHeader!();
    };
}

macro_rules! SectionTable {
    () => {
        deps!();
        # [doc = " The table of section headers in an XCOFF file."] # [doc = ""] # [doc = " Returned by [`FileHeader::sections`]."] # [derive (Debug , Clone , Copy)] pub struct SectionTable < 'data , Xcoff : FileHeader > { sections : & 'data [Xcoff :: SectionHeader] , }
    };
}

SectionTable!()