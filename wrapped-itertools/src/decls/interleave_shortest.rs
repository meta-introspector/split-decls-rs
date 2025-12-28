macro_rules! deps {
    () => {
        InterleaveShortest!();
    };
}

macro_rules! interleave_shortest {
    () => {
        deps!();
        # [doc = " Create a new `InterleaveShortest` iterator."] pub fn interleave_shortest < I , J > (i : I , j : J) -> InterleaveShortest < I , J > where I : Iterator , J : Iterator < Item = I :: Item > , { InterleaveShortest { i , j , next_coming_from_j : false , } }
    };
}

interleave_shortest!()