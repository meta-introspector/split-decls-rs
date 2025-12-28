macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! unzip_into_vecs {
    () => {
        deps!();
        # [doc = " Unzips the results of the exact iterator into the specified vectors."] # [doc = ""] # [doc = " This is called by `IndexedParallelIterator::unzip_into_vecs`."] pub (super) fn unzip_into_vecs < I , A , B > (pi : I , left : & mut Vec < A > , right : & mut Vec < B >) where I : IndexedParallelIterator < Item = (A , B) > , A : Send , B : Send , { left . truncate (0) ; right . truncate (0) ; let len = pi . len () ; collect_with_consumer (right , len , | right_consumer | { let mut right_result = None ; collect_with_consumer (left , len , | left_consumer | { let (left_r , right_r) = unzip_indexed (pi , left_consumer , right_consumer) ; right_result = Some (right_r) ; left_r }) ; right_result . unwrap () }) ; }
    };
}

unzip_into_vecs!()