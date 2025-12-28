macro_rules! deps {
    () => {
        ReadRef!();
        MachOFile!();
        MachHeader!();
        MachOSegmentInternal!();
        ObjectSegment!();
    };
}

macro_rules! MachOSegment {
    () => {
        deps!();
        # [doc = " A segment in a [`MachOFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] # [derive (Debug)] pub struct MachOSegment < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { file : & 'file MachOFile < 'data , Mach , R > , internal : & 'file MachOSegmentInternal < 'data , Mach , R > , }
    };
}

MachOSegment!();