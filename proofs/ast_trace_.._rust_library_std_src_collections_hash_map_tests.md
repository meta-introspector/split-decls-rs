# AST Trace: ../rust/library/std/src/collections/hash/map/tests.rs

Generated 53 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use rand::Rng;
use realstd::collections::TryReserveErrorKind::*;

use super::Entry::{Occupied, Vacant};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use super::HashMap;
use crate::assert_matches::assert_matches;
use crate::cell::RefCell;
use crate::hash::{BuildHasher, BuildHasherDefault, DefaultHasher, RandomState};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=_assert_hashmap_is_unwind_safe | COMPLEXITY=3 | LINES=7

```rust
use crate::test_helpers::test_rng;

// https://github.com/rust-lang/rust/issues/62301
fn _assert_hashmap_is_unwind_safe() {
    fn assert_unwind_safe<T: crate::panic::UnwindSafe>() {}
    assert_unwind_safe::<HashMap<(), crate::cell::UnsafeCell<()>>>();
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=test_zero_capacities | COMPLEXITY=3 | LINES=32

```rust
#[test]
fn test_zero_capacities() {
    type HM = HashMap<i32, i32>;

    let m = HM::new();
    assert_eq!(m.capacity(), 0);

    let m = HM::default();
    assert_eq!(m.capacity(), 0);

    let m = HM::with_hasher(RandomState::new());
    assert_eq!(m.capacity(), 0);

    let m = HM::with_capacity(0);
    assert_eq!(m.capacity(), 0);

    let m = HM::with_capacity_and_hasher(0, RandomState::new());
    assert_eq!(m.capacity(), 0);

    let mut m = HM::new();
    m.insert(1, 1);
    m.insert(2, 2);
    m.remove(&1);
    m.remove(&2);
    m.shrink_to_fit();
    assert_eq!(m.capacity(), 0);

    let mut m = HM::new();
    m.reserve(0);
    assert_eq!(m.capacity(), 0);
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=test_create_capacity_zero | COMPLEXITY=2 | LINES=10

```rust
#[test]
fn test_create_capacity_zero() {
    let mut m = HashMap::with_capacity(0);

    assert!(m.insert(1, 1).is_none());

    assert!(m.contains_key(&1));
    assert!(!m.contains_key(&0));
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=test_insert | COMPLEXITY=2 | LINES=12

```rust
#[test]
fn test_insert() {
    let mut m = HashMap::new();
    assert_eq!(m.len(), 0);
    assert!(m.insert(1, 2).is_none());
    assert_eq!(m.len(), 1);
    assert!(m.insert(2, 4).is_none());
    assert_eq!(m.len(), 2);
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&2).unwrap(), 4);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=test_clone | COMPLEXITY=2 | LINES=14

```rust
#[test]
fn test_clone() {
    let mut m = HashMap::new();
    assert_eq!(m.len(), 0);
    assert!(m.insert(1, 2).is_none());
    assert_eq!(m.len(), 1);
    assert!(m.insert(2, 4).is_none());
    assert_eq!(m.len(), 2);
    let m2 = m.clone();
    assert_eq!(*m2.get(&1).unwrap(), 2);
    assert_eq!(*m2.get(&2).unwrap(), 4);
    assert_eq!(m2.len(), 2);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
thread_local! { static DROP_VECTOR: RefCell<Vec<i32>> = RefCell::new(Vec::new()) }
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=Droppable | COMPLEXITY=2 | LINES=5

```rust
#[derive(Hash, PartialEq, Eq)]
struct Droppable {
    k: usize,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=new | COMPLEXITY=5 | LINES=10

```rust
impl Droppable {
    fn new(k: usize) -> Droppable {
        DROP_VECTOR.with(|slot| {
            slot.borrow_mut()[k] += 1;
        });

        Droppable { k }
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=6 | LINES=8

```rust
impl Drop for Droppable {
    fn drop(&mut self) {
        DROP_VECTOR.with(|slot| {
            slot.borrow_mut()[self.k] -= 1;
        });
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=5 | LINES=6

```rust
impl Clone for Droppable {
    fn clone(&self) -> Droppable {
        Droppable::new(self.k)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=test_drops | COMPLEXITY=32 | LINES=59

```rust
#[test]
fn test_drops() {
    DROP_VECTOR.with(|slot| {
        *slot.borrow_mut() = vec![0; 200];
    });

    {
        let mut m = HashMap::new();

        DROP_VECTOR.with(|v| {
            for i in 0..200 {
                assert_eq!(v.borrow()[i], 0);
            }
        });

        for i in 0..100 {
            let d1 = Droppable::new(i);
            let d2 = Droppable::new(i + 100);
            m.insert(d1, d2);
        }

        DROP_VECTOR.with(|v| {
            for i in 0..200 {
                assert_eq!(v.borrow()[i], 1);
            }
        });

        for i in 0..50 {
            let k = Droppable::new(i);
            let v = m.remove(&k);

            assert!(v.is_some());

            DROP_VECTOR.with(|v| {
                assert_eq!(v.borrow()[i], 1);
                assert_eq!(v.borrow()[i + 100], 1);
            });
        }

        DROP_VECTOR.with(|v| {
            for i in 0..50 {
                assert_eq!(v.borrow()[i], 0);
                assert_eq!(v.borrow()[i + 100], 0);
            }

            for i in 50..100 {
                assert_eq!(v.borrow()[i], 1);
                assert_eq!(v.borrow()[i + 100], 1);
            }
        });
    }

    DROP_VECTOR.with(|v| {
        for i in 0..200 {
            assert_eq!(v.borrow()[i], 0);
        }
    });
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=test_into_iter_drops | COMPLEXITY=30 | LINES=61

```rust
#[test]
fn test_into_iter_drops() {
    DROP_VECTOR.with(|v| {
        *v.borrow_mut() = vec![0; 200];
    });

    let hm = {
        let mut hm = HashMap::new();

        DROP_VECTOR.with(|v| {
            for i in 0..200 {
                assert_eq!(v.borrow()[i], 0);
            }
        });

        for i in 0..100 {
            let d1 = Droppable::new(i);
            let d2 = Droppable::new(i + 100);
            hm.insert(d1, d2);
        }

        DROP_VECTOR.with(|v| {
            for i in 0..200 {
                assert_eq!(v.borrow()[i], 1);
            }
        });

        hm
    };

    // By the way, ensure that cloning doesn't screw up the dropping.
    drop(hm.clone());

    {
        let mut half = hm.into_iter().take(50);

        DROP_VECTOR.with(|v| {
            for i in 0..200 {
                assert_eq!(v.borrow()[i], 1);
            }
        });

        for _ in half.by_ref() {}

        DROP_VECTOR.with(|v| {
            let nk = (0..100).filter(|&i| v.borrow()[i] == 1).count();

            let nv = (0..100).filter(|&i| v.borrow()[i + 100] == 1).count();

            assert_eq!(nk, 50);
            assert_eq!(nv, 50);
        });
    };

    DROP_VECTOR.with(|v| {
        for i in 0..200 {
            assert_eq!(v.borrow()[i], 0);
        }
    });
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=test_empty_remove | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn test_empty_remove() {
    let mut m: HashMap<i32, bool> = HashMap::new();
    assert_eq!(m.remove(&0), None);
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=test_empty_entry | COMPLEXITY=7 | LINES=11

```rust
#[test]
fn test_empty_entry() {
    let mut m: HashMap<i32, bool> = HashMap::new();
    match m.entry(0) {
        Occupied(_) => panic!(),
        Vacant(_) => {}
    }
    assert!(*m.entry(0).or_insert(true));
    assert_eq!(m.len(), 1);
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=test_empty_iter | COMPLEXITY=2 | LINES=14

```rust
#[test]
fn test_empty_iter() {
    let mut m: HashMap<i32, bool> = HashMap::new();
    assert_eq!(m.drain().next(), None);
    assert_eq!(m.keys().next(), None);
    assert_eq!(m.values().next(), None);
    assert_eq!(m.values_mut().next(), None);
    assert_eq!(m.iter().next(), None);
    assert_eq!(m.iter_mut().next(), None);
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
    assert_eq!(m.into_iter().next(), None);
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=test_lots_of_insertions | COMPLEXITY=52 | LINES=66

```rust
#[test]
fn test_lots_of_insertions() {
    let mut m = HashMap::new();

    // Try this a few times to make sure we never screw up the hashmap's
    // internal state.
    let loops = if cfg!(miri) { 2 } else { 10 };
    for _ in 0..loops {
        assert!(m.is_empty());

        let count = if cfg!(miri) { 66 } else { 1001 };

        for i in 1..count {
            assert!(m.insert(i, i).is_none());

            for j in 1..=i {
                let r = m.get(&j);
                assert_eq!(r, Some(&j));
            }

            for j in i + 1..count {
                let r = m.get(&j);
                assert_eq!(r, None);
            }
        }

        for i in count..(2 * count) {
            assert!(!m.contains_key(&i));
        }

        // remove forwards
        for i in 1..count {
            assert!(m.remove(&i).is_some());

            for j in 1..=i {
                assert!(!m.contains_key(&j));
            }

            for j in i + 1..count {
                assert!(m.contains_key(&j));
            }
        }

        for i in 1..count {
            assert!(!m.contains_key(&i));
        }

        for i in 1..count {
            assert!(m.insert(i, i).is_none());
        }

        // remove backwards
        for i in (1..count).rev() {
            assert!(m.remove(&i).is_some());

            for j in i..count {
                assert!(!m.contains_key(&j));
            }

            for j in 1..i {
                assert!(m.contains_key(&j));
            }
        }
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=test_find_mut | COMPLEXITY=6 | LINES=14

```rust
#[test]
fn test_find_mut() {
    let mut m = HashMap::new();
    assert!(m.insert(1, 12).is_none());
    assert!(m.insert(2, 8).is_none());
    assert!(m.insert(5, 14).is_none());
    let new = 100;
    match m.get_mut(&5) {
        None => panic!(),
        Some(x) => *x = new,
    }
    assert_eq!(m.get(&5), Some(&new));
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=test_insert_overwrite | COMPLEXITY=2 | LINES=9

```rust
#[test]
fn test_insert_overwrite() {
    let mut m = HashMap::new();
    assert!(m.insert(1, 2).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert!(!m.insert(1, 3).is_none());
    assert_eq!(*m.get(&1).unwrap(), 3);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=test_insert_conflicts | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn test_insert_conflicts() {
    let mut m = HashMap::with_capacity(4);
    assert!(m.insert(1, 2).is_none());
    assert!(m.insert(5, 3).is_none());
    assert!(m.insert(9, 4).is_none());
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert_eq!(*m.get(&1).unwrap(), 2);
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=test_conflict_remove | COMPLEXITY=3 | LINES=17

```rust
#[test]
fn test_conflict_remove() {
    let mut m = HashMap::with_capacity(4);
    assert!(m.insert(1, 2).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert!(m.insert(5, 3).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert!(m.insert(9, 4).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert!(m.remove(&1).is_some());
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert_eq!(*m.get(&5).unwrap(), 3);
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=test_is_empty | COMPLEXITY=2 | LINES=9

```rust
#[test]
fn test_is_empty() {
    let mut m = HashMap::with_capacity(4);
    assert!(m.insert(1, 2).is_none());
    assert!(!m.is_empty());
    assert!(m.remove(&1).is_some());
    assert!(m.is_empty());
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=test_remove | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn test_remove() {
    let mut m = HashMap::new();
    m.insert(1, 2);
    assert_eq!(m.remove(&1), Some(2));
    assert_eq!(m.remove(&1), None);
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=test_remove_entry | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn test_remove_entry() {
    let mut m = HashMap::new();
    m.insert(1, 2);
    assert_eq!(m.remove_entry(&1), Some((1, 2)));
    assert_eq!(m.remove(&1), None);
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=test_iterate | COMPLEXITY=8 | LINES=17

```rust
#[test]
fn test_iterate() {
    let mut m = HashMap::with_capacity(4);
    for i in 0..32 {
        assert!(m.insert(i, i * 2).is_none());
    }
    assert_eq!(m.len(), 32);

    let mut observed: u32 = 0;

    for (k, v) in &m {
        assert_eq!(*v, *k * 2);
        observed |= 1 << *k;
    }
    assert_eq!(observed, 0xFFFF_FFFF);
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=test_keys | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn test_keys() {
    let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    let keys: Vec<_> = map.keys().cloned().collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&3));
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=test_values | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn test_values() {
    let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&'a'));
    assert!(values.contains(&'b'));
    assert!(values.contains(&'c'));
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=test_values_mut | COMPLEXITY=5 | LINES=14

```rust
#[test]
fn test_values_mut() {
    let pairs = [(1, 1), (2, 2), (3, 3)];
    let mut map: HashMap<_, _> = pairs.into_iter().collect();
    for value in map.values_mut() {
        *value = (*value) * 2
    }
    let values: Vec<_> = map.values().cloned().collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&2));
    assert!(values.contains(&4));
    assert!(values.contains(&6));
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=test_into_keys | COMPLEXITY=2 | LINES=12

```rust
#[test]
fn test_into_keys() {
    let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    let keys: Vec<_> = map.into_keys().collect();

    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&3));
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=test_into_values | COMPLEXITY=2 | LINES=12

```rust
#[test]
fn test_into_values() {
    let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    let values: Vec<_> = map.into_values().collect();

    assert_eq!(values.len(), 3);
    assert!(values.contains(&'a'));
    assert!(values.contains(&'b'));
    assert!(values.contains(&'c'));
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=test_find | COMPLEXITY=6 | LINES=11

```rust
#[test]
fn test_find() {
    let mut m = HashMap::new();
    assert!(m.get(&1).is_none());
    m.insert(1, 2);
    match m.get(&1) {
        None => panic!(),
        Some(v) => assert_eq!(*v, 2),
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=test_eq | COMPLEXITY=2 | LINES=18

```rust
#[test]
fn test_eq() {
    let mut m1 = HashMap::new();
    m1.insert(1, 2);
    m1.insert(2, 3);
    m1.insert(3, 4);

    let mut m2 = HashMap::new();
    m2.insert(1, 2);
    m2.insert(2, 3);

    assert!(m1 != m2);

    m2.insert(3, 4);

    assert_eq!(m1, m2);
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=test_show | COMPLEXITY=7 | LINES=14

```rust
#[test]
fn test_show() {
    let mut map = HashMap::new();
    let empty: HashMap<i32, i32> = HashMap::new();

    map.insert(1, 2);
    map.insert(3, 4);

    let map_str = format!("{map:?}");

    assert!(map_str == "{1: 2, 3: 4}" || map_str == "{3: 4, 1: 2}");
    assert_eq!(format!("{empty:?}"), "{}");
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=test_reserve_shrink_to_fit | COMPLEXITY=15 | LINES=37

```rust
#[test]
fn test_reserve_shrink_to_fit() {
    let mut m = HashMap::new();
    m.insert(0, 0);
    m.remove(&0);
    assert!(m.capacity() >= m.len());
    for i in 0..128 {
        m.insert(i, i);
    }
    m.reserve(256);

    let usable_cap = m.capacity();
    for i in 128..(128 + 256) {
        m.insert(i, i);
        assert_eq!(m.capacity(), usable_cap);
    }

    for i in 100..(128 + 256) {
        assert_eq!(m.remove(&i), Some(i));
    }
    m.shrink_to_fit();

    assert_eq!(m.len(), 100);
    assert!(!m.is_empty());
    assert!(m.capacity() >= m.len());

    for i in 0..100 {
        assert_eq!(m.remove(&i), Some(i));
    }
    m.shrink_to_fit();
    m.insert(0, 0);

    assert_eq!(m.len(), 1);
    assert!(m.capacity() >= m.len());
    assert_eq!(m.remove(&0), Some(0));
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=test_from_iter | COMPLEXITY=5 | LINES=13

```rust
#[test]
fn test_from_iter() {
    let xs = [(1, 1), (2, 2), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

    let map: HashMap<_, _> = xs.iter().cloned().collect();

    for &(k, v) in &xs {
        assert_eq!(map.get(&k), Some(&v));
    }

    assert_eq!(map.iter().len(), xs.len() - 1);
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=test_size_hint | COMPLEXITY=5 | LINES=13

```rust
#[test]
fn test_size_hint() {
    let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

    let map: HashMap<_, _> = xs.iter().cloned().collect();

    let mut iter = map.iter();

    for _ in iter.by_ref().take(3) {}

    assert_eq!(iter.size_hint(), (3, Some(3)));
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=test_iter_len | COMPLEXITY=5 | LINES=13

```rust
#[test]
fn test_iter_len() {
    let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

    let map: HashMap<_, _> = xs.iter().cloned().collect();

    let mut iter = map.iter();

    for _ in iter.by_ref().take(3) {}

    assert_eq!(iter.len(), 3);
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=test_mut_size_hint | COMPLEXITY=5 | LINES=13

```rust
#[test]
fn test_mut_size_hint() {
    let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

    let mut map: HashMap<_, _> = xs.iter().cloned().collect();

    let mut iter = map.iter_mut();

    for _ in iter.by_ref().take(3) {}

    assert_eq!(iter.size_hint(), (3, Some(3)));
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=test_iter_mut_len | COMPLEXITY=5 | LINES=13

```rust
#[test]
fn test_iter_mut_len() {
    let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

    let mut map: HashMap<_, _> = xs.iter().cloned().collect();

    let mut iter = map.iter_mut();

    for _ in iter.by_ref().take(3) {}

    assert_eq!(iter.len(), 3);
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=test_index | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn test_index() {
    let mut map = HashMap::new();

    map.insert(1, 2);
    map.insert(2, 1);
    map.insert(3, 4);

    assert_eq!(map[&2], 1);
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=test_index_nonexistent | COMPLEXITY=2 | LINES=12

```rust
#[test]
#[should_panic]
fn test_index_nonexistent() {
    let mut map = HashMap::new();

    map.insert(1, 2);
    map.insert(2, 1);
    map.insert(3, 4);

    map[&4];
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=test_entry | COMPLEXITY=24 | LINES=50

```rust
#[test]
fn test_entry() {
    let xs = [(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)];

    let mut map: HashMap<_, _> = xs.iter().cloned().collect();

    // Existing key (insert)
    match map.entry(1) {
        Vacant(_) => unreachable!(),
        Occupied(mut view) => {
            assert_eq!(view.get(), &10);
            assert_eq!(view.insert(100), 10);
        }
    }
    assert_eq!(map.get(&1).unwrap(), &100);
    assert_eq!(map.len(), 6);

    // Existing key (update)
    match map.entry(2) {
        Vacant(_) => unreachable!(),
        Occupied(mut view) => {
            let v = view.get_mut();
            let new_v = (*v) * 10;
            *v = new_v;
        }
    }
    assert_eq!(map.get(&2).unwrap(), &200);
    assert_eq!(map.len(), 6);

    // Existing key (take)
    match map.entry(3) {
        Vacant(_) => unreachable!(),
        Occupied(view) => {
            assert_eq!(view.remove(), 30);
        }
    }
    assert_eq!(map.get(&3), None);
    assert_eq!(map.len(), 5);

    // Inexistent key (insert)
    match map.entry(10) {
        Occupied(_) => unreachable!(),
        Vacant(view) => {
            assert_eq!(*view.insert(1000), 1000);
        }
    }
    assert_eq!(map.get(&10).unwrap(), &1000);
    assert_eq!(map.len(), 6);
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=test_entry_take_doesnt_corrupt | COMPLEXITY=22 | LINES=32

```rust
#[test]
fn test_entry_take_doesnt_corrupt() {
    #![allow(deprecated)] //rand
    // Test for #19292
    fn check(m: &HashMap<i32, ()>) {
        for k in m.keys() {
            assert!(m.contains_key(k), "{k} is in keys() but not in the map?");
        }
    }

    let mut m = HashMap::new();
    let mut rng = test_rng();

    // Populate the map with some items.
    for _ in 0..50 {
        let x = rng.gen_range(-10..10);
        m.insert(x, ());
    }

    for _ in 0..1000 {
        let x = rng.gen_range(-10..10);
        match m.entry(x) {
            Vacant(_) => {}
            Occupied(e) => {
                e.remove();
            }
        }

        check(&m);
    }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=test_extend_ref | COMPLEXITY=2 | LINES=16

```rust
#[test]
fn test_extend_ref() {
    let mut a = HashMap::new();
    a.insert(1, "one");
    let mut b = HashMap::new();
    b.insert(2, "two");
    b.insert(3, "three");

    a.extend(&b);

    assert_eq!(a.len(), 3);
    assert_eq!(a[&1], "one");
    assert_eq!(a[&2], "two");
    assert_eq!(a[&3], "three");
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=test_capacity_not_less_than_len | COMPLEXITY=8 | LINES=25

```rust
#[test]
fn test_capacity_not_less_than_len() {
    let mut a = HashMap::new();
    let mut item = 0;

    for _ in 0..116 {
        a.insert(item, 0);
        item += 1;
    }

    assert!(a.capacity() > a.len());

    let free = a.capacity() - a.len();
    for _ in 0..free {
        a.insert(item, 0);
        item += 1;
    }

    assert_eq!(a.len(), a.capacity());

    // Insert at capacity should cause allocation.
    a.insert(item, 0);
    assert!(a.capacity() > a.len());
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=test_occupied_entry_key | COMPLEXITY=6 | LINES=18

```rust
#[test]
fn test_occupied_entry_key() {
    let mut a = HashMap::new();
    let key = "hello there";
    let value = "value goes here";
    assert!(a.is_empty());
    a.insert(key, value);
    assert_eq!(a.len(), 1);
    assert_eq!(a[key], value);

    match a.entry(key) {
        Vacant(_) => panic!(),
        Occupied(e) => assert_eq!(key, *e.key()),
    }
    assert_eq!(a.len(), 1);
    assert_eq!(a[key], value);
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=test_vacant_entry_key | COMPLEXITY=7 | LINES=18

```rust
#[test]
fn test_vacant_entry_key() {
    let mut a = HashMap::new();
    let key = "hello there";
    let value = "value goes here";

    assert!(a.is_empty());
    match a.entry(key) {
        Occupied(_) => panic!(),
        Vacant(e) => {
            assert_eq!(key, *e.key());
            e.insert(value);
        }
    }
    assert_eq!(a.len(), 1);
    assert_eq!(a[key], value);
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=test_retain | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn test_retain() {
    let mut map: HashMap<i32, i32> = (0..100).map(|x| (x, x * 10)).collect();

    map.retain(|&k, _| k % 2 == 0);
    assert_eq!(map.len(), 50);
    assert_eq!(map[&2], 20);
    assert_eq!(map[&4], 40);
    assert_eq!(map[&6], 60);
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=test_try_reserve | COMPLEXITY=12 | LINES=31

```rust
#[test]
#[cfg_attr(miri, ignore)] // Miri does not support signalling OOM
#[cfg_attr(target_os = "android", ignore)] // Android used in CI has a broken dlmalloc
fn test_try_reserve() {
    let mut empty_bytes: HashMap<u8, u8> = HashMap::new();

    const MAX_USIZE: usize = usize::MAX;

    assert_matches!(
        empty_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
        Err(CapacityOverflow),
        "usize::MAX should trigger an overflow!"
    );

    if let Err(AllocError { .. }) = empty_bytes.try_reserve(MAX_USIZE / 16).map_err(|e| e.kind()) {
    } else {
        // This may succeed if there is enough free memory. Attempt to
        // allocate a few more hashmaps to ensure the allocation will fail.
        let mut empty_bytes2: HashMap<u8, u8> = HashMap::new();
        let _ = empty_bytes2.try_reserve(MAX_USIZE / 16);
        let mut empty_bytes3: HashMap<u8, u8> = HashMap::new();
        let _ = empty_bytes3.try_reserve(MAX_USIZE / 16);
        let mut empty_bytes4: HashMap<u8, u8> = HashMap::new();
        assert_matches!(
            empty_bytes4.try_reserve(MAX_USIZE / 16).map_err(|e| e.kind()),
            Err(AllocError { .. }),
            "usize::MAX / 16 should trigger an OOM!"
        );
    }
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=eq_sorted | COMPLEXITY=61 | LINES=165

```rust
mod test_extract_if {
    use super::*;
    use crate::panic::{AssertUnwindSafe, catch_unwind};
    use crate::sync::atomic::{AtomicUsize, Ordering};

    trait EqSorted: Iterator {
        fn eq_sorted<I: IntoIterator<Item = Self::Item>>(self, other: I) -> bool;
    }

    impl<T: Iterator> EqSorted for T
    where
        T::Item: Eq + Ord,
    {
        fn eq_sorted<I: IntoIterator<Item = Self::Item>>(self, other: I) -> bool {
            let mut v: Vec<_> = self.collect();
            v.sort_unstable();
            v.into_iter().eq(other)
        }
    }

    #[test]
    fn empty() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.extract_if(|_, _| unreachable!("there's nothing to decide on")).for_each(drop);
        assert!(map.is_empty());
    }

    #[test]
    fn consuming_nothing() {
        let pairs = (0..3).map(|i| (i, i));
        let mut map: HashMap<_, _> = pairs.collect();
        assert!(map.extract_if(|_, _| false).eq_sorted(crate::iter::empty()));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn consuming_all() {
        let pairs = (0..3).map(|i| (i, i));
        let mut map: HashMap<_, _> = pairs.clone().collect();
        assert!(map.extract_if(|_, _| true).eq_sorted(pairs));
        assert!(map.is_empty());
    }

    #[test]
    fn mutating_and_keeping() {
        let pairs = (0..3).map(|i| (i, i));
        let mut map: HashMap<_, _> = pairs.collect();
        assert!(
            map.extract_if(|_, v| {
                *v += 6;
                false
            })
            .eq_sorted(crate::iter::empty())
        );
        assert!(map.keys().copied().eq_sorted(0..3));
        assert!(map.values().copied().eq_sorted(6..9));
    }

    #[test]
    fn mutating_and_removing() {
        let pairs = (0..3).map(|i| (i, i));
        let mut map: HashMap<_, _> = pairs.collect();
        assert!(
            map.extract_if(|_, v| {
                *v += 6;
                true
            })
            .eq_sorted((0..3).map(|i| (i, i + 6)))
        );
        assert!(map.is_empty());
    }

    #[test]
    #[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
    fn drop_panic_leak() {
        static PREDS: AtomicUsize = AtomicUsize::new(0);
        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct D;
        impl Drop for D {
            fn drop(&mut self) {
                if DROPS.fetch_add(1, Ordering::SeqCst) == 1 {
                    panic!("panic in `drop`");
                }
            }
        }

        let mut map = (0..3).map(|i| (i, D)).collect::<HashMap<_, _>>();

        catch_unwind(move || {
            map.extract_if(|_, _| {
                PREDS.fetch_add(1, Ordering::SeqCst);
                true
            })
            .for_each(drop)
        })
        .unwrap_err();

        assert_eq!(PREDS.load(Ordering::SeqCst), 2);
        assert_eq!(DROPS.load(Ordering::SeqCst), 3);
    }

    #[test]
    #[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
    fn pred_panic_leak() {
        static PREDS: AtomicUsize = AtomicUsize::new(0);
        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct D;
        impl Drop for D {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        let mut map = (0..3).map(|i| (i, D)).collect::<HashMap<_, _>>();

        catch_unwind(AssertUnwindSafe(|| {
            map.extract_if(|_, _| match PREDS.fetch_add(1, Ordering::SeqCst) {
                0 => true,
                _ => panic!(),
            })
            .for_each(drop)
        }))
        .unwrap_err();

        assert_eq!(PREDS.load(Ordering::SeqCst), 2);
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
        assert_eq!(map.len(), 2);
    }

    // Same as above, but attempt to use the iterator again after the panic in the predicate
    #[test]
    #[cfg_attr(not(panic = "unwind"), ignore = "test requires unwinding support")]
    fn pred_panic_reuse() {
        static PREDS: AtomicUsize = AtomicUsize::new(0);
        static DROPS: AtomicUsize = AtomicUsize::new(0);

        struct D;
        impl Drop for D {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        let mut map = (0..3).map(|i| (i, D)).collect::<HashMap<_, _>>();

        {
            let mut it = map.extract_if(|_, _| match PREDS.fetch_add(1, Ordering::SeqCst) {
                0 => true,
                _ => panic!(),
            });
            catch_unwind(AssertUnwindSafe(|| while it.next().is_some() {})).unwrap_err();
            // Iterator behavior after a panic is explicitly unspecified,
            // so this is just the current implementation:
            let result = catch_unwind(AssertUnwindSafe(|| it.next()));
            assert!(result.is_err());
        }

        assert_eq!(PREDS.load(Ordering::SeqCst), 3);
        assert_eq!(DROPS.load(Ordering::SeqCst), 1);
        assert_eq!(map.len(), 2);
    }
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=from_array | COMPLEXITY=2 | LINES=12

```rust
#[test]
fn from_array() {
    let map = HashMap::from([(1, 2), (3, 4)]);
    let unordered_duplicates = HashMap::from([(3, 4), (1, 2), (1, 2)]);
    assert_eq!(map, unordered_duplicates);

    // This next line must infer the hasher type parameter.
    // If you make a change that causes this line to no longer infer,
    // that's a problem!
    let _must_not_require_type_annotation = HashMap::from([(1, 2)]);
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=const_with_hasher | COMPLEXITY=7 | LINES=26

```rust
#[test]
fn const_with_hasher() {
    const X: HashMap<(), (), BuildHasherDefault<DefaultHasher>> =
        HashMap::with_hasher(BuildHasherDefault::new());
    let mut x = X;
    assert_eq!(x.len(), 0);
    x.insert((), ());
    assert_eq!(x.len(), 1);

    // It *is* possible to do this without using the `BuildHasherDefault` type.
    struct MyBuildDefaultHasher;
    impl BuildHasher for MyBuildDefaultHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    const Y: HashMap<(), (), MyBuildDefaultHasher> = HashMap::with_hasher(MyBuildDefaultHasher);
    let mut y = Y;
    assert_eq!(y.len(), 0);
    y.insert((), ());
    assert_eq!(y.len(), 1);
}
```

---
*Generated by AST tracing system*
