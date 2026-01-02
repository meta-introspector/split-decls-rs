mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: slice :: sort :: stable :: merge ;}

macro_rules! mergesort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mergesort in module {}", module_path!());
    };
}

mkfn!{
    mergesort_introspect!();
    # [doc = " Tiny recursive top-down merge sort optimized for binary size. It has no adaptiveness whatsoever,"] # [doc = " no run detection, etc."] # [inline (always)] pub fn mergesort < T , F : FnMut (& T , & T) -> bool > (v : & mut [T] , scratch : & mut [MaybeUninit < T >] , is_less : & mut F ,) { let len = v . len () ; if len > 2 { let mid = len / 2 ; unsafe { mergesort (v . get_unchecked_mut (.. mid) , scratch , is_less) ; mergesort (v . get_unchecked_mut (mid ..) , scratch , is_less) ; } merge :: merge (v , scratch , mid , is_less) ; } else if len == 2 { unsafe { let v_base = v . as_mut_ptr () ; let v_a = v_base ; let v_b = v_base . add (1) ; if is_less (& * v_b , & * v_a) { ptr :: swap_nonoverlapping (v_a , v_b , 1) ; } } } }
}