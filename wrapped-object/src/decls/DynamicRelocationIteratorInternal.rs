macro_rules! deps {
    () => {
        ReadRef!();
        Endianness!();
        ElfDynamicRelocationIterator32!();
        ElfDynamicRelocationIterator64!();
    };
}

macro_rules! DynamicRelocationIteratorInternal {
    () => {
        deps!();
        # [derive (Debug)] enum DynamicRelocationIteratorInternal < 'data , 'file , R > where R : ReadRef < 'data > , { # [cfg (feature = "elf")] Elf32 (elf :: ElfDynamicRelocationIterator32 < 'data , 'file , Endianness , R >) , # [cfg (feature = "elf")] Elf64 (elf :: ElfDynamicRelocationIterator64 < 'data , 'file , Endianness , R >) , # [allow (unused)] None (PhantomData < (& 'data () , & 'file () , R) >) , }
    };
}

DynamicRelocationIteratorInternal!()