macro_rules! deps {
    () => {
        Symbols!();
        VersionFiles!();
        Sections!();
        Segments!();
        Versions!();
        ByteString!();
        Endianness!();
        Header!();
        DynamicSymbols!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " A builder for reading, modifying, and then writing ELF files."] # [doc = ""] # [doc = " Public fields are available for modifying the values that will be written."] # [doc = " Methods are available to add elements to tables, and elements can be deleted"] # [doc = " from tables by setting the `delete` field in the element."] # [derive (Debug)] pub struct Builder < 'data > { # [doc = " The endianness."] # [doc = ""] # [doc = " Used to set the data encoding when writing the ELF file."] pub endian : Endianness , # [doc = " Whether file is 64-bit."] # [doc = ""] # [doc = " Use to set the file class when writing the ELF file."] pub is_64 : bool , # [doc = " The alignment of [`elf::PT_LOAD`] segments."] # [doc = ""] # [doc = " This is an informational field and is not used when writing the ELF file."] # [doc = " It can optionally be used when calling [`Segments::add_load_segment`]."] # [doc = ""] # [doc = " It is determined heuristically when reading the ELF file. Currently,"] # [doc = " if all load segments have the same alignment, that alignment is used,"] # [doc = " otherwise it is set to 1."] pub load_align : u64 , # [doc = " The file header."] pub header : Header , # [doc = " The segment table."] pub segments : Segments < 'data > , # [doc = " The section table."] pub sections : Sections < 'data > , # [doc = " The symbol table."] pub symbols : Symbols < 'data > , # [doc = " The dynamic symbol table."] pub dynamic_symbols : DynamicSymbols < 'data > , # [doc = " The base version for the GNU version definitions."] # [doc = ""] # [doc = " This will be written as a version definition with index 1."] pub version_base : Option < ByteString < 'data > > , # [doc = " The GNU version definitions and dependencies."] pub versions : Versions < 'data > , # [doc = " The filenames used in the GNU version definitions."] pub version_files : VersionFiles < 'data > , # [doc = " The bucket count parameter for the hash table."] pub hash_bucket_count : u32 , # [doc = " The bloom shift parameter for the GNU hash table."] pub gnu_hash_bloom_shift : u32 , # [doc = " The bloom count parameter for the GNU hash table."] pub gnu_hash_bloom_count : u32 , # [doc = " The bucket count parameter for the GNU hash table."] pub gnu_hash_bucket_count : u32 , marker : PhantomData < () > , }
    };
}

Builder!();