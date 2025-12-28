macro_rules! heapsort {
    () => {
        # [doc = " Sorts `v` using heapsort, which guarantees *O*(*n* \\* log(*n*)) worst-case."] # [cold] fn heapsort < T , F > (v : & mut [T] , is_less : F) where F : Fn (& T , & T) -> bool , { let sift_down = | v : & mut [T] , mut node | { loop { let mut child = 2 * node + 1 ; if child >= v . len () { break ; } if child + 1 < v . len () { child += is_less (& v [child] , & v [child + 1]) as usize ; } if ! is_less (& v [node] , & v [child]) { break ; } v . swap (node , child) ; node = child ; } } ; for i in (0 .. v . len () / 2) . rev () { sift_down (v , i) ; } for i in (1 .. v . len ()) . rev () { v . swap (0 , i) ; sift_down (& mut v [.. i] , 0) ; } }
    };
}

heapsort!()