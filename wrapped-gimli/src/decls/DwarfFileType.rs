macro_rules! DwarfFileType {
    () => {
        # [doc = " The \"type\" of file with DWARF debugging information. This determines, among other things,"] # [doc = " which files DWARF sections should be loaded from."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Default)] pub enum DwarfFileType { # [doc = " A normal executable or object file."] # [default] Main , # [doc = " A .dwo split DWARF file."] Dwo , }
    };
}

DwarfFileType!()