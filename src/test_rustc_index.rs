// Test case for rustc_index import issue
use crate::rustc_index::bit_set::DenseBitSet;
use crate::rustc_index::{Idx, IndexVec};

pub fn test_rustc_index_imports() {
    // This should compile if rustc_index module is properly accessible
    let _dense_bit_set: DenseBitSet;
    let _idx: Idx;
    let _index_vec: IndexVec;
    println!("rustc_index imports work!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_imports() {
        test_rustc_index_imports();
    }
}
