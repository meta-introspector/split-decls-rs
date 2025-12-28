macro_rules! deps {
    () => {
        CallFrameInstructionIter!();
        CommonInformationEntry!();
        BaseAddresses!();
        Pointer!();
        Reader!();
        Section!();
        FrameDescriptionEntry!();
        PointerEncodingParameters!();
        UnwindSection!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        # [doc = " # Signal Safe Methods"] # [doc = ""] # [doc = " These methods are guaranteed not to allocate, acquire locks, or perform any"] # [doc = " other signal-unsafe operations."] # [allow (clippy :: len_without_is_empty)] impl < R : Reader > FrameDescriptionEntry < R > { # [doc = " Get the offset of this entry from the start of its containing section."] pub fn offset (& self) -> R :: Offset { self . offset } # [doc = " Get a reference to this FDE's CIE."] pub fn cie (& self) -> & CommonInformationEntry < R > { & self . cie } # [doc = " > A constant that gives the number of bytes of the header and"] # [doc = " > instruction stream for this function, not including the length field"] # [doc = " > itself (see Section 7.2.2). The size of the length field plus the value"] # [doc = " > of length must be an integral multiple of the address size."] pub fn entry_len (& self) -> R :: Offset { self . length } # [doc = " Iterate over this FDE's instructions."] # [doc = ""] # [doc = " Will not include the CIE's initial instructions, if you want those do"] # [doc = " `fde.cie().instructions()` first."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn instructions < 'a , Section > (& self , section : & 'a Section , bases : & 'a BaseAddresses ,) -> CallFrameInstructionIter < 'a , R > where Section : UnwindSection < R > , { CallFrameInstructionIter { input : self . instructions . clone () , address_encoding : self . cie . augmentation () . and_then (| a | a . fde_address_encoding) , parameters : PointerEncodingParameters { bases : & bases . eh_frame , func_base : None , address_size : self . cie . address_size , section : section . section () , } , vendor : section . vendor () , } } # [doc = " The first address for which this entry has unwind information for."] pub fn initial_address (& self) -> u64 { self . initial_address } # [doc = " One more than the last address that this entry has unwind information for."] # [doc = ""] # [doc = " This uses wrapping arithmetic, so the result may be less than"] # [doc = " `initial_address`."] pub fn end_address (& self) -> u64 { self . initial_address . wrapping_add_sized (self . address_range , self . cie . address_size) } # [doc = " The number of bytes of instructions that this entry has unwind"] # [doc = " information for."] pub fn len (& self) -> u64 { self . address_range } # [doc = " Return `true` if the given address is within this FDE, `false`"] # [doc = " otherwise."] # [doc = ""] # [doc = " This is equivalent to `entry.initial_address() <= address <"] # [doc = " entry.initial_address() + entry.len()`."] pub fn contains (& self , address : u64) -> bool { self . initial_address () <= address && address < self . end_address () } # [doc = " The address of this FDE's language-specific data area (LSDA), if it has"] # [doc = " any."] pub fn lsda (& self) -> Option < Pointer > { self . augmentation . as_ref () . and_then (| a | a . lsda) } # [doc = " Return true if this FDE's function is a trampoline for a signal handler."] # [inline] pub fn is_signal_trampoline (& self) -> bool { self . cie () . is_signal_trampoline () } # [doc = " Return the address of the FDE's function's personality routine"] # [doc = " handler. The personality routine does language-specific clean up when"] # [doc = " unwinding the stack frames with the intent to not run them again."] # [inline] pub fn personality (& self) -> Option < Pointer > { self . cie () . personality () } }
    };
}

impl_209!();