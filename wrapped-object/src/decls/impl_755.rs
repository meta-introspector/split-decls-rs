macro_rules! deps {
    () => {
        WasmRelocationIterator!();
        ObjectSection!();
        Result!();
        WasmSection!();
        Import!();
        SectionFlags!();
        CompressedData!();
        SectionId!();
        SectionIndex!();
        RelocationMap!();
        RelocationIterator!();
        SectionKind!();
        CompressedFileRange!();
        ReadRef!();
        Table!();
        Export!();
    };
}

macro_rules! impl_755 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > ObjectSection < 'data > for WasmSection < 'data , 'file , R > { type RelocationIterator = WasmRelocationIterator < 'data , 'file , R > ; # [inline] fn index (& self) -> SectionIndex { SectionIndex (self . section . id as usize) } # [inline] fn address (& self) -> u64 { 0 } # [inline] fn size (& self) -> u64 { let range = & self . section . range ; (range . end - range . start) as u64 } # [inline] fn align (& self) -> u64 { 1 } # [inline] fn file_range (& self) -> Option < (u64 , u64) > { let range = & self . section . range ; Some ((range . start as _ , range . end as _)) } # [inline] fn data (& self) -> Result < & 'data [u8] > { let range = & self . section . range ; self . file . data . read_bytes_at (range . start as u64 , range . end as u64 - range . start as u64) . read_error ("Invalid Wasm section size or offset") } fn data_range (& self , _address : u64 , _size : u64) -> Result < Option < & 'data [u8] > > { unimplemented ! () } # [inline] fn compressed_file_range (& self) -> Result < CompressedFileRange > { Ok (CompressedFileRange :: none (self . file_range ())) } # [inline] fn compressed_data (& self) -> Result < CompressedData < 'data > > { self . data () . map (CompressedData :: none) } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { self . name () . map (str :: as_bytes) } # [inline] fn name (& self) -> Result < & 'data str > { Ok (match self . section . id { SectionId :: Custom => self . section . name , SectionId :: Type => "<type>" , SectionId :: Import => "<import>" , SectionId :: Function => "<function>" , SectionId :: Table => "<table>" , SectionId :: Memory => "<memory>" , SectionId :: Global => "<global>" , SectionId :: Export => "<export>" , SectionId :: Start => "<start>" , SectionId :: Element => "<element>" , SectionId :: Code => "<code>" , SectionId :: Data => "<data>" , SectionId :: DataCount => "<data_count>" , SectionId :: Tag => "<tag>" , }) } # [inline] fn segment_name_bytes (& self) -> Result < Option < & [u8] > > { Ok (None) } # [inline] fn segment_name (& self) -> Result < Option < & str > > { Ok (None) } # [inline] fn kind (& self) -> SectionKind { match self . section . id { SectionId :: Custom => match self . section . name { "reloc." | "linking" => SectionKind :: Linker , _ => SectionKind :: Other , } , SectionId :: Type => SectionKind :: Metadata , SectionId :: Import => SectionKind :: Linker , SectionId :: Function => SectionKind :: Metadata , SectionId :: Table => SectionKind :: UninitializedData , SectionId :: Memory => SectionKind :: UninitializedData , SectionId :: Global => SectionKind :: Data , SectionId :: Export => SectionKind :: Linker , SectionId :: Start => SectionKind :: Linker , SectionId :: Element => SectionKind :: Data , SectionId :: Code => SectionKind :: Text , SectionId :: Data => SectionKind :: Data , SectionId :: DataCount => SectionKind :: UninitializedData , SectionId :: Tag => SectionKind :: Data , } } # [inline] fn relocations (& self) -> WasmRelocationIterator < 'data , 'file , R > { WasmRelocationIterator (PhantomData) } fn relocation_map (& self) -> read :: Result < RelocationMap > { RelocationMap :: new (self . file , self) } # [inline] fn flags (& self) -> SectionFlags { SectionFlags :: None } }
    };
}

impl_755!();