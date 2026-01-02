mkuse!{use crate :: { hint , intrinsics } ;}
mkitem!{const PSEUDO_MEDIAN_REC_THRESHOLD : usize = 64 ;}

macro_rules! choose_pivot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function choose_pivot in module {}", module_path!());
    };
}

mkfn!{
    choose_pivot_introspect!();
    # [doc = " Selects a pivot from `v`. Algorithm taken from glidesort by Orson Peters."] # [doc = ""] # [doc = " This chooses a pivot by sampling an adaptive amount of points, approximating"] # [doc = " the quality of a median of sqrt(n) elements."] # [inline] pub fn choose_pivot < T , F : FnMut (& T , & T) -> bool > (v : & [T] , is_less : & mut F) -> usize { let len = v . len () ; if len < 8 { intrinsics :: abort () ; } let index = unsafe { let v_base = v . as_ptr () ; let len_div_8 = len / 8 ; let a = v_base ; let b = v_base . add (len_div_8 * 4) ; let c = v_base . add (len_div_8 * 7) ; if len < PSEUDO_MEDIAN_REC_THRESHOLD { median3 (& * a , & * b , & * c , is_less) . offset_from_unsigned (v_base) } else { median3_rec (a , b , c , len_div_8 , is_less) . offset_from_unsigned (v_base) } } ; unsafe { hint :: assert_unchecked (index < v . len ()) ; index } }
}

macro_rules! median3_rec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function median3_rec in module {}", module_path!());
    };
}

mkfn!{
    median3_rec_introspect!();
    # [doc = " Calculates an approximate median of 3 elements from sections a, b, c, or"] # [doc = " recursively from an approximation of each, if they're large enough. By"] # [doc = " dividing the size of each section by 8 when recursing we have logarithmic"] # [doc = " recursion depth and overall sample from f(n) = 3*f(n/8) -> f(n) ="] # [doc = " O(n^(log(3)/log(8))) ~= O(n^0.528) elements."] # [doc = ""] # [doc = " SAFETY: a, b, c must point to the start of initialized regions of memory of"] # [doc = " at least n elements."] unsafe fn median3_rec < T , F : FnMut (& T , & T) -> bool > (mut a : * const T , mut b : * const T , mut c : * const T , n : usize , is_less : & mut F ,) -> * const T { unsafe { if n * 8 >= PSEUDO_MEDIAN_REC_THRESHOLD { let n8 = n / 8 ; a = median3_rec (a , a . add (n8 * 4) , a . add (n8 * 7) , n8 , is_less) ; b = median3_rec (b , b . add (n8 * 4) , b . add (n8 * 7) , n8 , is_less) ; c = median3_rec (c , c . add (n8 * 4) , c . add (n8 * 7) , n8 , is_less) ; } median3 (& * a , & * b , & * c , is_less) } }
}

macro_rules! median3_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function median3 in module {}", module_path!());
    };
}

mkfn!{
    median3_introspect!();
    # [doc = " Calculates the median of 3 elements."] # [doc = ""] # [doc = " SAFETY: a, b, c must be valid initialized elements."] # [inline (always)] fn median3 < T , F : FnMut (& T , & T) -> bool > (a : & T , b : & T , c : & T , is_less : & mut F) -> * const T { let x = is_less (a , b) ; let y = is_less (a , c) ; if x == y { let z = is_less (b , c) ; if z ^ x { c } else { b } } else { a } }
}