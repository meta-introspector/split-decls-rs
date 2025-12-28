macro_rules! heapify {
    () => {
        # [doc = " Make `data` a heap (min-heap w.r.t the sorting)."] fn heapify < T , S > (data : & mut [T] , mut less_than : S) where S : FnMut (& T , & T) -> bool , { for i in (0 .. data . len () / 2) . rev () { sift_down (data , i , & mut less_than) ; } }
    };
}

heapify!()