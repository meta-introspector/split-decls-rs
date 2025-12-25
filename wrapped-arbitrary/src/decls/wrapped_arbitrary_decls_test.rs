use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn exhausted_entropy() {
        let mut u = Unstructured::new(&[]);
        assert_eq!(u.arbitrary::<bool>().unwrap(), false);
        assert_eq!(u.arbitrary::<u8>().unwrap(), 0);
        assert_eq!(u.arbitrary::<usize>().unwrap(), 0);
        assert_eq!(u.arbitrary::<f32>().unwrap(), 0.0);
        assert_eq!(u.arbitrary::<f64>().unwrap(), 0.0);
        assert_eq!(u.arbitrary::<Option<u32>>().unwrap(), None);
        assert_eq!(u.int_in_range(4..=100).unwrap(), 4);
        assert_eq!(u.choose_index(10).unwrap(), 0);
        assert_eq!(u.ratio(5, 7).unwrap(), true);
    }
}
