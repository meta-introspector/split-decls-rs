macro_rules! deps {
    () => {
        AuxHeader!();
        AuxHeader32!();
    };
}

macro_rules! impl_785 {
    () => {
        deps!();
        impl AuxHeader for xcoff :: AuxHeader32 { type Word = u32 ; fn o_mflag (& self) -> u16 { self . o_mflag . get (BE) } fn o_vstamp (& self) -> u16 { self . o_vstamp . get (BE) } fn o_tsize (& self) -> Self :: Word { self . o_tsize . get (BE) } fn o_dsize (& self) -> Self :: Word { self . o_dsize . get (BE) } fn o_bsize (& self) -> Self :: Word { self . o_bsize . get (BE) } fn o_entry (& self) -> Self :: Word { self . o_entry . get (BE) } fn o_text_start (& self) -> Self :: Word { self . o_text_start . get (BE) } fn o_data_start (& self) -> Self :: Word { self . o_data_start . get (BE) } fn o_toc (& self) -> Self :: Word { self . o_toc . get (BE) } fn o_snentry (& self) -> u16 { self . o_snentry . get (BE) } fn o_sntext (& self) -> u16 { self . o_sntext . get (BE) } fn o_sndata (& self) -> u16 { self . o_sndata . get (BE) } fn o_sntoc (& self) -> u16 { self . o_sntoc . get (BE) } fn o_snloader (& self) -> u16 { self . o_snloader . get (BE) } fn o_snbss (& self) -> u16 { self . o_snbss . get (BE) } fn o_algntext (& self) -> u16 { self . o_algntext . get (BE) } fn o_algndata (& self) -> u16 { self . o_algndata . get (BE) } fn o_modtype (& self) -> u16 { self . o_modtype . get (BE) } fn o_cpuflag (& self) -> u8 { self . o_cpuflag } fn o_cputype (& self) -> u8 { self . o_cputype } fn o_maxstack (& self) -> Self :: Word { self . o_maxstack . get (BE) } fn o_maxdata (& self) -> Self :: Word { self . o_maxdata . get (BE) } fn o_debugger (& self) -> u32 { self . o_debugger . get (BE) } fn o_textpsize (& self) -> u8 { self . o_textpsize } fn o_datapsize (& self) -> u8 { self . o_datapsize } fn o_stackpsize (& self) -> u8 { self . o_stackpsize } fn o_flags (& self) -> u8 { self . o_flags } fn o_sntdata (& self) -> u16 { self . o_sntdata . get (BE) } fn o_sntbss (& self) -> u16 { self . o_sntbss . get (BE) } fn o_x64flags (& self) -> Option < u16 > { None } }
    };
}

impl_785!()