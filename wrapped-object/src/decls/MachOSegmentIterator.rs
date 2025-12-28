macro_rules! deps {
    () => {
        MachOSegmentInternal!();
        MachOFile!();
        MachHeader!();
        ReadRef!();
    };
}

macro_rules! MachOSegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the segments in a [`MachOFile`]."] # [derive (Debug)] pub struct MachOSegmentIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , pub (super) iter : slice :: Iter < 'file , MachOSegmentInternal < 'data , Mach , R > > , }
    };
}

MachOSegmentIterator!()