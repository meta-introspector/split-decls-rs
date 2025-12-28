macro_rules! deps {
    () => {
        SentenceIndices!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'a > Iterator for SentenceIndices < 'a > { type Item = (usize , usize , & 'a str) ; # [inline] fn next (& mut self) -> Option < (usize , usize , & 'a str) > { let index = self . forward_index ; let (word , size) = decode_sentence (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; self . forward_index += size ; Some ((index , index + size , word)) } }
    };
}

impl_194!();