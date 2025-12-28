macro_rules! deps {
    () => {
        Segment!();
        MachHeader!();
        ReadRef!();
    };
}

macro_rules! MachOSegmentInternal {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] pub (super) struct MachOSegmentInternal < 'data , Mach : MachHeader , R : ReadRef < 'data > > { pub segment : & 'data Mach :: Segment , # [doc = " The data for the file that contains the segment data."] # [doc = ""] # [doc = " This is required for dyld caches, where this may be a different subcache"] # [doc = " from the file containing the Mach-O load commands."] pub data : R , }
    };
}

MachOSegmentInternal!();