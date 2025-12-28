macro_rules! deps {
    () => {
        File!();
        Bytes!();
        U16!();
        U32!();
    };
}

macro_rules! ImageDosHeader {
    () => {
        deps!();
        # [doc = " DOS .EXE header"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDosHeader { # [doc = " Magic number"] pub e_magic : U16 < LE > , # [doc = " Bytes on last page of file"] pub e_cblp : U16 < LE > , # [doc = " Pages in file"] pub e_cp : U16 < LE > , # [doc = " Relocations"] pub e_crlc : U16 < LE > , # [doc = " Size of header in paragraphs"] pub e_cparhdr : U16 < LE > , # [doc = " Minimum extra paragraphs needed"] pub e_minalloc : U16 < LE > , # [doc = " Maximum extra paragraphs needed"] pub e_maxalloc : U16 < LE > , # [doc = " Initial (relative) SS value"] pub e_ss : U16 < LE > , # [doc = " Initial SP value"] pub e_sp : U16 < LE > , # [doc = " Checksum"] pub e_csum : U16 < LE > , # [doc = " Initial IP value"] pub e_ip : U16 < LE > , # [doc = " Initial (relative) CS value"] pub e_cs : U16 < LE > , # [doc = " File address of relocation table"] pub e_lfarlc : U16 < LE > , # [doc = " Overlay number"] pub e_ovno : U16 < LE > , # [doc = " Reserved words"] pub e_res : [U16 < LE > ; 4] , # [doc = " OEM identifier (for e_oeminfo)"] pub e_oemid : U16 < LE > , # [doc = " OEM information; e_oemid specific"] pub e_oeminfo : U16 < LE > , # [doc = " Reserved words"] pub e_res2 : [U16 < LE > ; 10] , # [doc = " File address of new exe header"] pub e_lfanew : U32 < LE > , }
    };
}

ImageDosHeader!();