macro_rules! deps {
    () => {
        UnitTable!();
        LineProgram!();
    };
}

macro_rules! Dwarf {
    () => {
        deps!();
        # [doc = " Writable DWARF information for more than one unit."] # [derive (Debug , Default)] pub struct Dwarf { # [doc = " A table of units. These are primarily stored in the `.debug_info` section,"] # [doc = " but they also contain information that is stored in other sections."] pub units : UnitTable , # [doc = " Extra line number programs that are not associated with a unit."] # [doc = ""] # [doc = " These should only be used when generating DWARF5 line-only debug"] # [doc = " information."] pub line_programs : Vec < LineProgram > , # [doc = " A table of strings that will be stored in the `.debug_line_str` section."] pub line_strings : LineStringTable , # [doc = " A table of strings that will be stored in the `.debug_str` section."] pub strings : StringTable , }
    };
}

Dwarf!();