macro_rules! deps {
    () => {
        CsectAux64!();
        CsectAux!();
    };
}

macro_rules! impl_833 {
    () => {
        deps!();
        impl CsectAux for xcoff :: CsectAux64 { fn x_scnlen (& self) -> u64 { self . x_scnlen_lo . get (BE) as u64 | ((self . x_scnlen_hi . get (BE) as u64) << 32) } fn x_parmhash (& self) -> u32 { self . x_parmhash . get (BE) } fn x_snhash (& self) -> u16 { self . x_snhash . get (BE) } fn x_smtyp (& self) -> u8 { self . x_smtyp } fn x_smclas (& self) -> u8 { self . x_smclas } fn x_stab (& self) -> Option < u32 > { None } fn x_snstab (& self) -> Option < u16 > { None } fn x_auxtype (& self) -> Option < u8 > { Some (self . x_auxtype) } }
    };
}

impl_833!()