// Generated macro for print_attributes (function)
macro_rules! Depcrate_readobj_elfprint_attributes {
() => {
// Module: crate::readobj::elf
// Provides: {"print_attributes"}
// Dependencies: {}
fn print_attributes < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , section : & Elf :: SectionHeader ,) { if ! p . options . elf_attributes { return ; } if let Some (section) = section . attributes (endian , data) . print_err (p) { p . group ("Attributes" , | p | { p . field ("Version" , section . version ()) ; if let Some (mut subsections) = section . subsections () . print_err (p) { while let Some (Some (subsection)) = subsections . next () . print_err (p) { p . group ("Subsection" , | p | { p . field_inline_string ("Vendor" , subsection . vendor ()) ; let mut subsubsections = subsection . subsubsections () ; while let Some (Some (subsubsection)) = subsubsections . next () . print_err (p) { p . group ("Subsubsection" , | p | { p . field_enum ("Tag" , subsubsection . tag () , FLAGS_TAG) ; let mut indices = subsubsection . indices () ; while let Some (Some (index)) = indices . next () . print_err (p) { p . field ("Index" , index) ; } }) ; } }) ; } } }) ; } }
};
}
