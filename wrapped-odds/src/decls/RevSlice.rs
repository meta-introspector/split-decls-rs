macro_rules! RevSlice {
    () => {
        # [doc = " A reversed view of a slice."] # [doc = ""] # [doc = " The `RevSlice` is a random accessible range of elements;"] # [doc = " it wraps a regular slice but presents the underlying elements in"] # [doc = " reverse order."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use odds::slice::RevSlice;"] # [doc = ""] # [doc = " let mut data = [0; 8];"] # [doc = ""] # [doc = " {"] # [doc = "     let mut rev = <&mut RevSlice<_>>::from(&mut data);"] # [doc = "     for (i, elt) in rev.iter_mut().enumerate() {"] # [doc = "         *elt = i;"] # [doc = "     }"] # [doc = ""] # [doc = "     assert_eq!(&rev[..4], &[0, 1, 2, 3][..]);"] # [doc = " }"] # [doc = " assert_eq!(&data, &[7, 6, 5, 4, 3, 2, 1, 0]);"] # [doc = " ```"] # [doc = ""] # [doc = " Not visible in rustdoc:"] # [doc = ""] # [doc = " - A boxed slice can be reversed too:"] # [doc = "   `impl<T> From<Box<[T]>> for Box<RevSlice<T>>`."] # [derive (Debug , Eq)] # [repr (transparent)] pub struct RevSlice < T > ([T]) ;
    };
}

RevSlice!();