macro_rules! deps {
    () => {
        Segments!();
        Segment!();
        SegmentId!();
    };
}

macro_rules! impl_1117 {
    () => {
        deps!();
        impl < 'data > Segments < 'data > { # [doc = " Add a new segment to the table."] pub fn add (& mut self) -> & mut Segment < 'data > { let id = self . next_id () ; self . push (Segment { id , delete : false , p_type : 0 , p_flags : 0 , p_offset : 0 , p_vaddr : 0 , p_paddr : 0 , p_filesz : 0 , p_memsz : 0 , p_align : 0 , sections : Vec :: new () , marker : PhantomData , }) ; self . get_mut (id) } # [doc = " Find a `PT_LOAD` segment containing the given offset."] pub fn find_load_segment_from_offset (& self , offset : u64) -> Option < & Segment < 'data > > { self . iter () . find (| segment | segment . is_load () && segment . contains_offset (offset)) } # [doc = " Add a new `PT_LOAD` segment to the table."] # [doc = ""] # [doc = " The file offset and address will be derived from the current maximum for any segment."] # [doc = " The address will be chosen so that `p_paddr % align == p_offset % align`."] # [doc = " You may wish to use [`Builder::load_align`] for the alignment."] pub fn add_load_segment (& mut self , flags : u32 , align : u64) -> & mut Segment < 'data > { let mut max_offset = 0 ; let mut max_addr = 0 ; for segment in & * self { let offset = segment . p_offset + segment . p_filesz ; if max_offset < offset { max_offset = offset ; } let addr = segment . p_vaddr + segment . p_memsz ; if max_addr < addr { max_addr = addr ; } } let offset = max_offset ; let addr = ((max_addr + (align - 1)) & ! (align - 1)) + (offset & (align - 1)) ; let segment = self . add () ; segment . p_type = elf :: PT_LOAD ; segment . p_flags = flags ; segment . p_offset = offset ; segment . p_vaddr = addr ; segment . p_paddr = addr ; segment . p_align = align ; segment } # [doc = " Add a copy of a segment to the table."] # [doc = ""] # [doc = " This will copy the segment type, flags and alignment."] # [doc = ""] # [doc = " Additionally, if the segment type is `PT_LOAD`, then the file offset and address"] # [doc = " will be set as in `add_load_segment`."] pub fn copy (& mut self , id : SegmentId) -> & mut Segment < 'data > { let segment = self . get (id) ; let p_type = segment . p_type ; let p_flags = segment . p_flags ; let p_align = segment . p_align ; if p_type == elf :: PT_LOAD { self . add_load_segment (p_flags , p_align) } else { let segment = self . add () ; segment . p_type = p_type ; segment . p_flags = p_flags ; segment . p_align = p_align ; segment } } }
    };
}

impl_1117!();