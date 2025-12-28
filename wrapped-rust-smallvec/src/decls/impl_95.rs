macro_rules! deps {
    () => {
        ExtractIf!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        # [cfg (feature = "extract_if")] impl < T , F , const N : usize > Iterator for ExtractIf < '_ , T , N , F > where F : FnMut (& mut T) -> bool , { type Item = T ; fn next (& mut self) -> Option < T > { unsafe { while self . idx < self . end { let i = self . idx ; let v = core :: slice :: from_raw_parts_mut (self . vec . as_mut_ptr () , self . old_len) ; let drained = (self . pred) (& mut v [i]) ; self . idx += 1 ; if drained { self . del += 1 ; return Some (core :: ptr :: read (& v [i])) ; } else if self . del > 0 { let del = self . del ; let src : * const T = & v [i] ; let dst : * mut T = & mut v [i - del] ; core :: ptr :: copy_nonoverlapping (src , dst , 1) ; } } None } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . end - self . idx)) } }
    };
}

impl_95!();