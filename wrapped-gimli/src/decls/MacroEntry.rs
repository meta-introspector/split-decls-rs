macro_rules! deps {
    () => {
        MacroString!();
        DebugMacroOffset!();
        ReaderOffset!();
        Reader!();
    };
}

macro_rules! MacroEntry {
    () => {
        deps!();
        # [doc = " an Entry in the `.debug_macro` section."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum MacroEntry < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " A macro definition."] Define { # [doc = " The line number where the macro is defined."] line : u64 , # [doc = " The text of the macro: The name of the macro followed immediately by any formal"] # [doc = " parameters including the surrounding parentheses, followed by the macro definition."] text : MacroString < R > , } , # [doc = " A macro undefinition."] Undef { # [doc = " The line number where the macro is undefined."] line : u64 , # [doc = " The name of the macro without the definition."] name : MacroString < R > , } , # [doc = " The start of a file."] StartFile { # [doc = " Line number of the source file on which the inclusion macro directive occurred."] line : u64 , # [doc = " An index into the line number table of the compilation unit."] file : u64 , } , # [doc = " The end of the current included file."] EndFile , # [doc = " import a macro unit"] Import { # [doc = " offset of the macro unit in the `.debug_macro` section"] offset : DebugMacroOffset < Offset > , } , # [doc = " import a macro unit from the supplementary object file"] ImportSup { # [doc = " offset of the macro unit in the `.debug_macro` section of the supplementary object file"] offset : DebugMacroOffset < Offset > , } , # [doc = " A vendor-specific extension."] VendorExt { # [doc = " A numeric constant, whose meaning is vendor specific."] numeric : u64 , # [doc = " A string whose meaning is vendor specific."] string : R , } , }
    };
}

MacroEntry!();