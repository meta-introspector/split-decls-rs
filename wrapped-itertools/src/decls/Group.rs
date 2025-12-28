macro_rules! deps {
    () => {
        ChunkBy!();
    };
}

macro_rules! Group {
    () => {
        deps!();
        # [doc = " An iterator for the elements in a single group."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] pub struct Group < 'a , K , I , F > where I : Iterator + 'a , I :: Item : 'a , K : 'a , F : 'a , { parent : & 'a ChunkBy < K , I , F > , index : usize , first : Option < I :: Item > , }
    };
}

Group!()