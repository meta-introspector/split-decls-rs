use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod test {
    #[inline(never)]
    pub fn black_box<T>(val: T) -> T {
        use core::{mem, ptr};
        let ret = unsafe { ptr::read_volatile(&val) };
        mem::forget(val);
        ret
    }
    #[test]
    fn test_assembly() {
        use crate::functional::*;
        let a = black_box(arr![1, 3, 5, 7]);
        let b = black_box(arr![2, 4, 6, 8]);
        let c = (&a).zip(b, |l, r| l + r);
        let d = a.fold(0, |a, x| a + x);
        assert_eq!(c, arr![3, 7, 11, 15]);
        assert_eq!(d, 16);
    }
}
