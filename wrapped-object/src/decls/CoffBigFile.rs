macro_rules! deps {
    () => {
        CoffFile!();
        AnonObjectHeaderBigobj!();
        Object!();
    };
}

macro_rules! CoffBigFile {
    () => {
        deps!();
        # [doc = " A COFF bigobj object file with 32-bit section numbers."] # [doc = ""] # [doc = " This is a file that starts with [`pe::AnonObjectHeaderBigobj`], and corresponds"] # [doc = " to [`crate::FileKind::CoffBig`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] pub type CoffBigFile < 'data , R = & 'data [u8] > = CoffFile < 'data , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigFile!()