// Generated macro for Section (struct)
macro_rules! Depcrate_writeSection {
() => {
// Module: crate::write
// Provides: {"Section"}
// Dependencies: {}
# [doc = " A section in an object file."] # [derive (Debug)] pub struct Section < 'a > { segment : Vec < u8 > , name : Vec < u8 > , kind : SectionKind , size : u64 , align : u64 , data : Cow < 'a , [u8] > , relocations : Vec < Relocation > , symbol : Option < SymbolId > , # [doc = " Section flags that are specific to each file format."] pub flags : SectionFlags , }
};
}
