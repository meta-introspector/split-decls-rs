macro_rules! deps {
    () => {
        Result!();
        Bytes!();
        NodeIterator!();
        ExportSymbol!();
        ExportData!();
        Frame!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl < 'data > NodeIterator < 'data > { pub (super) fn new (data : & 'data [u8]) -> Self { NodeIterator { data , offset : 0 , stack : Vec :: new () , name_buf : Vec :: new () , } } fn push_node (& mut self) -> Result < Option < ExportSymbol < 'data > > > { let mut data = Bytes (& self . data . get (self . offset ..) . read_error ("Invalid exports trie offset") ? ,) ; let terminal_size = data . read_uleb128 () . read_error ("Invalid exports trie terminal size") ? ; let export_data = if terminal_size == 0 { None } else { let (flags , export_data) = ExportData :: parse (data . read_bytes (terminal_size as usize) . read_error ("Exports trie terminal size exceeds bounds") ? ,) ? ; Some (ExportSymbol { name : self . name_buf . clone () . into_boxed_slice () , flags , data : export_data , }) } ; let children_count = * data . read :: < u8 > () . read_error ("Invalid exports trie children count") ? ; self . stack . push (Frame { data , children_remaining : children_count , name_buf_len : self . name_buf . len () , }) ; Ok (export_data) } fn next (& mut self) -> Result < Option < Option < ExportSymbol < 'data > > > > { let Some (frame) = self . stack . last_mut () else { if self . offset == 0 { return Ok (Some (self . push_node () ?)) ; } return Ok (None) ; } ; self . name_buf . truncate (frame . name_buf_len) ; if frame . children_remaining == 0 { self . stack . pop () ; return self . next () ; } let edge_str = frame . data . read_string () . read_error ("Invalid exports trie edge string") ? ; let child_offset = frame . data . read_uleb128 () . read_error ("Invalid exports trie child offset") ? ; frame . children_remaining -= 1 ; self . name_buf . extend (edge_str) ; self . offset = child_offset as usize ; Ok (Some (self . push_node () ?)) } }
    };
}

impl_514!();