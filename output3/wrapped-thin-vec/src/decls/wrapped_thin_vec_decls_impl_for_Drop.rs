use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<A, F> Drop for ExtractIf<'_, A, F> {
    fn drop(&mut self) {
        unsafe {
            if self.idx < self.old_len && self.del > 0 {
                let ptr = self.vec.as_mut_ptr();
                let src = ptr.add(self.idx);
                let dst = src.sub(self.del);
                let tail_len = self.old_len - self.idx;
                src.copy_to(dst, tail_len);
            }
            self.vec.set_len(self.old_len - self.del);
        }
    }
}
