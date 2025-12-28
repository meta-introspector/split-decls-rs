macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Unzip!();
        UnzipConsumer!();
        Consumer!();
    };
}

macro_rules! unzip_indexed {
    () => {
        deps!();
        # [doc = " Unzips an `IndexedParallelIterator` into two arbitrary `Consumer`s."] # [doc = ""] # [doc = " This is called by `super::collect::unzip_into_vecs`."] pub (super) fn unzip_indexed < I , A , B , CA , CB > (pi : I , left : CA , right : CB) -> (CA :: Result , CB :: Result) where I : IndexedParallelIterator < Item = (A , B) > , CA : Consumer < A > , CB : Consumer < B > , A : Send , B : Send , { let consumer = UnzipConsumer { op : & Unzip , left , right , } ; pi . drive (consumer) }
    };
}

unzip_indexed!()