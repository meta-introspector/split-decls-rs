mkuse!{use crate :: mem :: { MaybeUninit , SizedTypeProperties } ;}
mkuse!{use crate :: ptr ;}
mkitem!{type BufType = [usize ; 32] ;}

macro_rules! ptr_rotate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ptr_rotate in module {}", module_path!());
    };
}

mkfn!{
    ptr_rotate_introspect!();
    # [doc = " Rotates the range `[mid-left, mid+right)` such that the element at `mid` becomes the first"] # [doc = " element. Equivalently, rotates the range `left` elements to the left or `right` elements to the"] # [doc = " right."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The specified range must be valid for reading and writing."] # [inline] pub (super) const unsafe fn ptr_rotate < T > (left : usize , mid : * mut T , right : usize) { if T :: IS_ZST { return ; } if (left == 0) || (right == 0) { return ; } if ! cfg ! (feature = "optimize_for_size") && const_min (left , right) <= size_of :: < BufType > () / size_of :: < T > () { unsafe { ptr_rotate_memmove (left , mid , right) } ; } else if ! cfg ! (feature = "optimize_for_size") && ((left + right < 24) || (size_of :: < T > () > size_of :: < [usize ; 4] > ())) { unsafe { ptr_rotate_gcd (left , mid , right) } } else { unsafe { ptr_rotate_swap (left , mid , right) } } }
}

macro_rules! ptr_rotate_memmove_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ptr_rotate_memmove in module {}", module_path!());
    };
}

mkfn!{
    ptr_rotate_memmove_introspect!();
    # [doc = " Algorithm 1 is used if `min(left, right)` is small enough to fit onto a stack buffer. The"] # [doc = " `min(left, right)` elements are copied onto the buffer, `memmove` is applied to the others, and"] # [doc = " the ones on the buffer are moved back into the hole on the opposite side of where they"] # [doc = " originated."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The specified range must be valid for reading and writing."] # [inline] const unsafe fn ptr_rotate_memmove < T > (left : usize , mid : * mut T , right : usize) { let mut rawarray = MaybeUninit :: < (BufType , [T ; 0]) > :: uninit () ; let buf = rawarray . as_mut_ptr () as * mut T ; let dim = unsafe { mid . sub (left) . add (right) } ; if left <= right { unsafe { ptr :: copy_nonoverlapping (mid . sub (left) , buf , left) ; ptr :: copy (mid , mid . sub (left) , right) ; ptr :: copy_nonoverlapping (buf , dim , left) ; } } else { unsafe { ptr :: copy_nonoverlapping (mid , buf , right) ; ptr :: copy (mid . sub (left) , dim , left) ; ptr :: copy_nonoverlapping (buf , mid . sub (left) , right) ; } } }
}

macro_rules! ptr_rotate_gcd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ptr_rotate_gcd in module {}", module_path!());
    };
}

mkfn!{
    ptr_rotate_gcd_introspect!();
    # [doc = " Algorithm 2 is used for small values of `left + right` or for large `T`. The elements"] # [doc = " are moved into their final positions one at a time starting at `mid - left` and advancing by"] # [doc = " `right` steps modulo `left + right`, such that only one temporary is needed. Eventually, we"] # [doc = " arrive back at `mid - left`. However, if `gcd(left + right, right)` is not 1, the above steps"] # [doc = " skipped over elements. For example:"] # [doc = " ```text"] # [doc = " left = 10, right = 6"] # [doc = " the `^` indicates an element in its final place"] # [doc = " 6 7 8 9 10 11 12 13 14 15 . 0 1 2 3 4 5"] # [doc = " after using one step of the above algorithm (The X will be overwritten at the end of the round,"] # [doc = " and 12 is stored in a temporary):"] # [doc = " X 7 8 9 10 11 6 13 14 15 . 0 1 2 3 4 5"] # [doc = "               ^"] # [doc = " after using another step (now 2 is in the temporary):"] # [doc = " X 7 8 9 10 11 6 13 14 15 . 0 1 12 3 4 5"] # [doc = "               ^                 ^"] # [doc = " after the third step (the steps wrap around, and 8 is in the temporary):"] # [doc = " X 7 2 9 10 11 6 13 14 15 . 0 1 12 3 4 5"] # [doc = "     ^         ^                 ^"] # [doc = " after 7 more steps, the round ends with the temporary 0 getting put in the X:"] # [doc = " 0 7 2 9 4 11 6 13 8 15 . 10 1 12 3 14 5"] # [doc = " ^   ^   ^    ^    ^       ^    ^    ^"] # [doc = " ```"] # [doc = " Fortunately, the number of skipped over elements between finalized elements is always equal, so"] # [doc = " we can just offset our starting position and do more rounds (the total number of rounds is the"] # [doc = " `gcd(left + right, right)` value). The end result is that all elements are finalized once and"] # [doc = " only once."] # [doc = ""] # [doc = " Algorithm 2 can be vectorized by chunking and performing many rounds at once, but there are too"] # [doc = " few rounds on average until `left + right` is enormous, and the worst case of a single"] # [doc = " round is always there."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The specified range must be valid for reading and writing."] # [inline] const unsafe fn ptr_rotate_gcd < T > (left : usize , mid : * mut T , right : usize) { let x = unsafe { mid . sub (left) } ; let mut tmp : T = unsafe { x . read () } ; let mut i = right ; let mut gcd = right ; loop { tmp = unsafe { x . add (i) . replace (tmp) } ; if i >= left { i -= left ; if i == 0 { unsafe { x . write (tmp) } ; break ; } if i < gcd { gcd = i ; } } else { i += right ; } } let mut start = 1 ; while start < gcd { tmp = unsafe { x . add (start) . read () } ; i = start + right ; loop { tmp = unsafe { x . add (i) . replace (tmp) } ; if i >= left { i -= left ; if i == start { unsafe { x . add (start) . write (tmp) } ; break ; } } else { i += right ; } } start += 1 ; } }
}

macro_rules! ptr_rotate_swap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ptr_rotate_swap in module {}", module_path!());
    };
}

mkfn!{
    ptr_rotate_swap_introspect!();
    # [doc = " Algorithm 3 utilizes repeated swapping of `min(left, right)` elements."] # [doc = ""] # [doc = " ///"] # [doc = " ```text"] # [doc = " left = 11, right = 4"] # [doc = " [4 5 6 7 8 9 10 11 12 13 14 . 0 1 2 3]"] # [doc = "                  ^  ^  ^  ^   ^ ^ ^ ^ swapping the right most elements with elements to the left"] # [doc = " [4 5 6 7 8 9 10 . 0 1 2 3] 11 12 13 14"] # [doc = "        ^ ^ ^  ^   ^ ^ ^ ^ swapping these"] # [doc = " [4 5 6 . 0 1 2 3] 7 8 9 10 11 12 13 14"] # [doc = " we cannot swap any more, but a smaller rotation problem is left to solve"] # [doc = " ```"] # [doc = " when `left < right` the swapping happens from the left instead."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The specified range must be valid for reading and writing."] # [inline] const unsafe fn ptr_rotate_swap < T > (mut left : usize , mut mid : * mut T , mut right : usize) { loop { if left >= right { loop { unsafe { ptr :: swap_nonoverlapping (mid . sub (right) , mid , right) ; mid = mid . sub (right) ; } left -= right ; if left < right { break ; } } } else { loop { unsafe { ptr :: swap_nonoverlapping (mid . sub (left) , mid , left) ; mid = mid . add (left) ; } right -= left ; if right < left { break ; } } } if (right == 0) || (left == 0) { return ; } } }
}

macro_rules! const_min_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_min in module {}", module_path!());
    };
}

mkfn!{
    const_min_introspect!();
    const fn const_min (left : usize , right : usize) -> usize { if right < left { right } else { left } }
}