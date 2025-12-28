macro_rules! from_iter_length_fail {
    () => {
        # [inline (never)] # [cold] pub (crate) fn from_iter_length_fail (length : usize) -> ! { panic ! ("GenericArray::from_iter expected {length} items") ; }
    };
}

from_iter_length_fail!()