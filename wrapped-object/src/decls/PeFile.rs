macro_rules! deps {
    () => {
        CoffCommon!();
        ImageNtHeaders!();
        ReadRef!();
        ImageDosHeader!();
        DataDirectories!();
        Object!();
    };
}

macro_rules! PeFile {
    () => {
        deps!();
        # [doc = " A PE image file."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] # [derive (Debug)] pub struct PeFile < 'data , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { pub (super) dos_header : & 'data pe :: ImageDosHeader , pub (super) nt_headers : & 'data Pe , pub (super) data_directories : DataDirectories < 'data > , pub (super) common : CoffCommon < 'data , R > , pub (super) data : R , }
    };
}

PeFile!();