macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! CodeView {
    () => {
        deps!();
        # [doc = " PDB information from the debug directory in a PE file."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct CodeView < 'data > { guid : [u8 ; 16] , path : ByteString < 'data > , age : u32 , }
    };
}

CodeView!()