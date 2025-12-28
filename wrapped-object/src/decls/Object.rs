macro_rules! deps {
    () => {
        StandardSection!();
        Symbol!();
        SubArchitecture!();
        MachOBuildVersion!();
        Comdat!();
        BinaryFormat!();
        File!();
        Section!();
        Mangling!();
        SymbolId!();
        Endianness!();
        Architecture!();
        SectionId!();
        FileFlags!();
    };
}

macro_rules! Object {
    () => {
        deps!();
        # [doc = " A writable relocatable object file."] # [derive (Debug)] pub struct Object < 'a > { format : BinaryFormat , architecture : Architecture , sub_architecture : Option < SubArchitecture > , endian : Endianness , sections : Vec < Section < 'a > > , standard_sections : HashMap < StandardSection , SectionId > , symbols : Vec < Symbol > , symbol_map : HashMap < Vec < u8 > , SymbolId > , comdats : Vec < Comdat > , # [doc = " File flags that are specific to each file format."] pub flags : FileFlags , # [doc = " The symbol name mangling scheme."] pub mangling : Mangling , # [cfg (feature = "coff")] stub_symbols : HashMap < SymbolId , SymbolId > , # [doc = " Mach-O \"_tlv_bootstrap\" symbol."] # [cfg (feature = "macho")] tlv_bootstrap : Option < SymbolId > , # [doc = " Mach-O CPU subtype."] # [cfg (feature = "macho")] macho_cpu_subtype : Option < u32 > , # [cfg (feature = "macho")] macho_build_version : Option < MachOBuildVersion > , # [doc = " Mach-O MH_SUBSECTIONS_VIA_SYMBOLS flag. Only ever set if format is Mach-O."] # [cfg (feature = "macho")] macho_subsections_via_symbols : bool , }
    };
}

Object!()