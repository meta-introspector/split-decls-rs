macro_rules! deps {
    () => {
        ReadRef!();
        ImageSectionHeader!();
        ObjectSegment!();
        CoffHeader!();
        CoffFile!();
        ImageFileHeader!();
    };
}

macro_rules! CoffSegment {
    () => {
        deps!();
        # [doc = " A loadable section in a [`CoffFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] # [derive (Debug)] pub struct CoffSegment < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) section : & 'data pe :: ImageSectionHeader , }
    };
}

CoffSegment!()