macro_rules! ChunkBySlice {
    () => {
        trait ChunkBySlice < T > : AsRef < [T] > + Default + Send { fn split (self , index : usize) -> (Self , Self) ; fn chunk_by (self , pred : & impl Fn (& T , & T) -> bool) -> impl Iterator < Item = Self > ; fn find (& self , pred : & impl Fn (& T , & T) -> bool , start : usize , end : usize) -> Option < usize > { self . as_ref () [start .. end] . windows (2) . position (move | w | ! pred (& w [0] , & w [1])) . map (| i | i + 1) } fn rfind (& self , pred : & impl Fn (& T , & T) -> bool , end : usize) -> Option < usize > { self . as_ref () [.. end] . windows (2) . rposition (move | w | ! pred (& w [0] , & w [1])) . map (| i | i + 1) } }
    };
}

ChunkBySlice!()