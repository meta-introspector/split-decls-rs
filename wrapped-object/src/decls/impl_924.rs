macro_rules! deps {
    () => {
        CodeView!();
    };
}

macro_rules! impl_924 {
    () => {
        deps!();
        impl < 'data > CodeView < 'data > { # [doc = " The path to the PDB as stored in CodeView."] # [inline] pub fn path (& self) -> & 'data [u8] { self . path . 0 } # [doc = " The age of the PDB."] # [inline] pub fn age (& self) -> u32 { self . age } # [doc = " The GUID of the PDB."] # [inline] pub fn guid (& self) -> [u8 ; 16] { self . guid } }
    };
}

impl_924!();