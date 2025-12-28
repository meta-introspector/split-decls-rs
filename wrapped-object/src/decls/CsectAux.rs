macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! CsectAux {
    () => {
        deps!();
        # [doc = " A trait for generic access to [`xcoff::CsectAux32`] and [`xcoff::CsectAux64`]."] # [allow (missing_docs)] pub trait CsectAux : Debug + Pod { fn x_scnlen (& self) -> u64 ; fn x_parmhash (& self) -> u32 ; fn x_snhash (& self) -> u16 ; fn x_smtyp (& self) -> u8 ; fn x_smclas (& self) -> u8 ; fn x_stab (& self) -> Option < u32 > ; fn x_snstab (& self) -> Option < u16 > ; fn x_auxtype (& self) -> Option < u8 > ; fn alignment (& self) -> u8 { self . x_smtyp () >> 3 } fn sym_type (& self) -> u8 { self . x_smtyp () & 0x07 } }
    };
}

CsectAux!();