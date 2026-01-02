mkuse!{use crate :: { cmp , intrinsics , ptr } ;}

macro_rules! heapsort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function heapsort in module {}", module_path!());
    };
}

mkfn!{
    heapsort_introspect!();
    # [doc = " Sorts `v` using heapsort, which guarantees *O*(*n* \\* log(*n*)) worst-case."] # [doc = ""] # [doc = " Never inline this, it sits the main hot-loop in `recurse` and is meant as unlikely algorithmic"] # [doc = " fallback."] # [inline (never)] pub (crate) fn heapsort < T , F > (v : & mut [T] , is_less : & mut F) where F : FnMut (& T , & T) -> bool , { let len = v . len () ; for i in (0 .. len + len / 2) . rev () { let sift_idx = if i >= len { i - len } else { v . swap (0 , i) ; 0 } ; unsafe { sift_down (& mut v [.. cmp :: min (i , len)] , sift_idx , is_less) ; } } }
}

macro_rules! sift_down_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sift_down in module {}", module_path!());
    };
}

mkfn!{
    sift_down_introspect!();
    # [inline (always)] unsafe fn sift_down < T , F > (v : & mut [T] , mut node : usize , is_less : & mut F) where F : FnMut (& T , & T) -> bool , { unsafe { intrinsics :: assume (node <= v . len ()) ; } let len = v . len () ; let v_base = v . as_mut_ptr () ; loop { let mut child = 2 * node + 1 ; if child >= len { break ; } unsafe { if child + 1 < len { child += is_less (& * v_base . add (child) , & * v_base . add (child + 1)) as usize ; } if ! is_less (& * v_base . add (node) , & * v_base . add (child)) { break ; } ptr :: swap_nonoverlapping (v_base . add (node) , v_base . add (child) , 1) ; } node = child ; } }
}