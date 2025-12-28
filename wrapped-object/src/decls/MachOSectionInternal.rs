macro_rules! deps {
    () => {
        MachHeader!();
        SectionKind!();
        Section!();
        ReadRef!();
        SectionIndex!();
    };
}

macro_rules! MachOSectionInternal {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] pub (super) struct MachOSectionInternal < 'data , Mach : MachHeader , R : ReadRef < 'data > > { pub index : SectionIndex , pub kind : SectionKind , pub section : & 'data Mach :: Section , # [doc = " The data for the file that contains the section data."] # [doc = ""] # [doc = " This is required for dyld caches, where this may be a different subcache"] # [doc = " from the file containing the Mach-O load commands."] pub data : R , }
    };
}

MachOSectionInternal!();