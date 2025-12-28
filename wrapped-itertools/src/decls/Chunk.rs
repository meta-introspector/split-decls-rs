macro_rules! deps {
    () => {
        IntoChunks!();
    };
}

macro_rules! Chunk {
    () => {
        deps!();
        # [doc = " An iterator for the elements in a single chunk."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] # [derive (Debug)] pub struct Chunk < 'a , I > where I : Iterator + 'a , I :: Item : 'a , { parent : & 'a IntoChunks < I > , index : usize , first : Option < I :: Item > , }
    };
}

Chunk!()