macro_rules! deps {
    () => {
        ReadRef!();
        FileHeader!();
        Object!();
        AuxHeader!();
        SectionTable!();
        SymbolTable!();
    };
}

macro_rules! XcoffFile {
    () => {
        deps!();
        # [doc = " A partially parsed XCOFF file."] # [doc = ""] # [doc = " Most functionality is provided by the [`Object`] trait implementation."] # [derive (Debug)] pub struct XcoffFile < 'data , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) data : R , pub (super) header : & 'data Xcoff , pub (super) aux_header : Option < & 'data Xcoff :: AuxHeader > , pub (super) sections : SectionTable < 'data , Xcoff > , pub (super) symbols : SymbolTable < 'data , Xcoff , R > , }
    };
}

XcoffFile!()