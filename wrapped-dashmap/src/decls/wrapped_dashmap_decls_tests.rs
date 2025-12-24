use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use crate::DashMap;
    use std::collections::hash_map::RandomState;
    #[test]
    fn test_basic() {
        let dm = DashMap::new();
        dm.insert(0, 0);
        assert_eq!(dm.get(& 0).unwrap().value(), & 0);
    }
    #[test]
    fn test_default() {
        let dm: DashMap<u32, u32> = DashMap::default();
        dm.insert(0, 0);
        assert_eq!(dm.get(& 0).unwrap().value(), & 0);
    }
    #[test]
    fn test_equal() {
        let dm1 = DashMap::new();
        let dm2 = DashMap::new();
        assert_eq!(dm1, dm2);
        dm1.insert(0, "Hello, world!");
        assert_ne!(dm1, dm2);
        dm1.insert(1, "Goodbye, world!");
        assert_ne!(dm1, dm2);
        dm2.insert(0, "Hello, world!");
        assert_ne!(dm1, dm2);
        dm2.insert(1, "Goodbye, world!");
        assert_eq!(dm1, dm2);
    }
    #[test]
    fn test_multiple_hashes() {
        let dm: DashMap<u32, u32> = DashMap::default();
        for i in 0..100 {
            dm.insert(0, i);
            dm.insert(i, i);
        }
        for i in 1..100 {
            let r = dm.get(&i).unwrap();
            assert_eq!(i, * r.value());
            assert_eq!(i, * r.key());
        }
        let r = dm.get(&0).unwrap();
        assert_eq!(99, * r.value());
    }
    #[test]
    fn test_more_complex_values() {
        #[derive(Hash, PartialEq, Debug, Clone)]
        struct T0 {
            s: String,
            u: u8,
        }
        let dm = DashMap::new();
        let range = 0..10;
        for i in range {
            let t = T0 { s: i.to_string(), u: i as u8 };
            dm.insert(i, t.clone());
            assert_eq!(& t, dm.get(& i).unwrap().value());
        }
    }
    #[test]
    fn test_different_hashers_randomstate() {
        let dm_hm_default: DashMap<u32, u32, RandomState> = DashMap::with_hasher(
            RandomState::new(),
        );
        for i in 0..10 {
            dm_hm_default.insert(i, i);
            assert_eq!(i, * dm_hm_default.get(& i).unwrap().value());
        }
    }
    #[test]
    fn test_map_view() {
        let dm = DashMap::new();
        let vegetables: [String; 4] = [
            "Salad".to_string(),
            "Beans".to_string(),
            "Potato".to_string(),
            "Tomato".to_string(),
        ];
        dm.insert(0, "Banana".to_string());
        dm.insert(4, "Pear".to_string());
        dm.insert(9, "Potato".to_string());
        dm.insert(12, "Chicken".to_string());
        let potato_vegetableness = dm.view(&9, |_, v| vegetables.contains(v));
        assert_eq!(potato_vegetableness, Some(true));
        let chicken_vegetableness = dm.view(&12, |_, v| vegetables.contains(v));
        assert_eq!(chicken_vegetableness, Some(false));
        let not_in_map = dm.view(&30, |_k, _v| false);
        assert_eq!(not_in_map, None);
    }
    #[test]
    fn test_try_get() {
        {
            let map = DashMap::new();
            map.insert("Johnny", 21);
            assert_eq!(* map.try_get("Johnny").unwrap(), 21);
            let _result1_locking = map.get_mut("Johnny");
            let result2 = map.try_get("Johnny");
            assert!(result2.is_locked());
        }
        {
            let map = DashMap::new();
            map.insert("Johnny", 21);
            *map.try_get_mut("Johnny").unwrap() += 1;
            assert_eq!(* map.get("Johnny").unwrap(), 22);
            let _result1_locking = map.get("Johnny");
            let result2 = map.try_get_mut("Johnny");
            assert!(result2.is_locked());
        }
    }
    #[test]
    fn test_try_reserve() {
        let mut map: DashMap<i32, i32> = DashMap::new();
        assert_eq!(map.capacity(), 0);
        map.try_reserve(10).unwrap();
        assert!(map.capacity() >= 10);
    }
    #[test]
    fn test_try_reserve_errors() {
        let mut map: DashMap<i32, i32> = DashMap::new();
        match map.try_reserve(usize::MAX) {
            Err(_) => {}
            _ => panic!("should have raised CapacityOverflow error"),
        }
    }
}
