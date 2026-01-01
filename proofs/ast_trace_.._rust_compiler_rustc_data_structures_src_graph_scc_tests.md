# AST Trace: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs

Generated 24 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=MaxReached(usize); | COMPLEXITY=7 | LINES=23

```rust
extern crate test;

use super::*;
use crate::graph::tests::TestGraph;

#[derive(Copy, Clone, Debug)]
struct MaxReached(usize);
struct Maxes(IndexVec<usize, MaxReached>, fn(usize) -> usize);
type UsizeSccs = Sccs<usize, usize>;

impl Annotations<usize> for Maxes {
    fn new(&self, element: usize) -> MaxReached {
        MaxReached(self.1(element))
    }

    fn annotate_scc(&mut self, scc: usize, annotation: MaxReached) {
        let i = self.0.push(annotation);
        assert!(i == scc);
    }

    type Ann = MaxReached;
    type SccIdx = usize;
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=annotation | COMPLEXITY=4 | LINES=9

```rust
impl Maxes {
    fn annotation(&self, scc: usize) -> MaxReached {
        self.0[scc]
    }
    fn new(mapping: fn(usize) -> usize) -> Self {
        Self(IndexVec::new(), mapping)
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=merge_scc | COMPLEXITY=6 | LINES=10

```rust
impl Annotation for MaxReached {
    fn merge_scc(self, other: Self) -> Self {
        Self(std::cmp::max(other.0, self.0))
    }

    fn merge_reached(self, other: Self) -> Self {
        Self(std::cmp::max(other.0, self.0))
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=5 | LINES=6

```rust
impl PartialEq<usize> for MaxReached {
    fn eq(&self, other: &usize) -> bool {
        &self.0 == other
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=MinMaxIn | COMPLEXITY=2 | LINES=6

```rust
#[derive(Copy, Clone, Debug)]
struct MinMaxIn {
    min: usize,
    max: usize,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=MinMaxes(IndexVec | COMPLEXITY=3 | LINES=7

```rust
struct MinMaxes(IndexVec<usize, MinMaxIn>, fn(usize) -> MinMaxIn);

impl MinMaxes {
    fn annotation(&self, scc: usize) -> MinMaxIn {
        self.0[scc]
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=new | COMPLEXITY=6 | LINES=14

```rust
impl Annotations<usize> for MinMaxes {
    fn new(&self, element: usize) -> MinMaxIn {
        self.1(element)
    }

    fn annotate_scc(&mut self, scc: usize, annotation: MinMaxIn) {
        let i = self.0.push(annotation);
        assert!(i == scc);
    }

    type Ann = MinMaxIn;
    type SccIdx = usize;
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=merge_scc | COMPLEXITY=7 | LINES=10

```rust
impl Annotation for MinMaxIn {
    fn merge_scc(self, other: Self) -> Self {
        Self { min: std::cmp::min(self.min, other.min), max: std::cmp::max(self.max, other.max) }
    }

    fn merge_reached(self, _other: Self) -> Self {
        self
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=diamond | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn diamond() {
    let graph = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
    let sccs: UsizeSccs = Sccs::new(&graph);
    assert_eq!(sccs.num_sccs(), 4);
    assert_eq!(sccs.num_sccs(), 4);
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=test_big_scc | COMPLEXITY=3 | LINES=27

```rust
#[test]
fn test_big_scc() {
    // The order in which things will be visited is important to this
    // test.
    //
    // We will visit:
    //
    // 0 -> 1 -> 2 -> 0
    //
    // and at this point detect a cycle. 2 will return back to 1 which
    // will visit 3. 3 will visit 2 before the cycle is complete, and
    // hence it too will return a cycle.

    /*
    +-> 0
    |   |
    |   v
    |   1 -> 3
    |   |    |
    |   v    |
    +-- 2 <--+
         */
    let graph = TestGraph::new(0, &[(0, 1), (1, 2), (1, 3), (2, 0), (3, 2)]);
    let sccs: UsizeSccs = Sccs::new(&graph);
    assert_eq!(sccs.num_sccs(), 1);
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=test_three_sccs | COMPLEXITY=3 | LINES=23

```rust
#[test]
fn test_three_sccs() {
    /*
        0
        |
        v
    +-> 1    3
    |   |    |
    |   v    |
    +-- 2 <--+
         */
    let graph = TestGraph::new(0, &[(0, 1), (1, 2), (2, 1), (3, 2)]);
    let sccs: UsizeSccs = Sccs::new(&graph);
    assert_eq!(sccs.num_sccs(), 3);
    assert_eq!(sccs.scc(0), 1);
    assert_eq!(sccs.scc(1), 0);
    assert_eq!(sccs.scc(2), 0);
    assert_eq!(sccs.scc(3), 2);
    assert_eq!(sccs.successors(0), &[] as &[usize]);
    assert_eq!(sccs.successors(1), &[0]);
    assert_eq!(sccs.successors(2), &[0]);
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=test_find_state_2 | COMPLEXITY=16 | LINES=55

```rust
#[test]
fn test_find_state_2() {
    // The order in which things will be visited is important to this
    // test. It tests part of the `find_state` behavior. Here is the
    // graph:
    //
    //
    //       /----+
    //     0 <--+ |
    //     |    | |
    //     v    | |
    // +-> 1 -> 3 4
    // |   |      |
    // |   v      |
    // +-- 2 <----+

    let graph = TestGraph::new(0, &[(0, 1), (0, 4), (1, 2), (1, 3), (2, 1), (3, 0), (4, 2)]);

    // For this graph, we will start in our DFS by visiting:
    //
    // 0 -> 1 -> 2 -> 1
    //
    // and at this point detect a cycle. The state of 2 will thus be
    // `InCycleWith { 1 }`. We will then visit the 1 -> 3 edge, which
    // will attempt to visit 0 as well, thus going to the state
    // `InCycleWith { 0 }`. Finally, node 1 will complete; the lowest
    // depth of any successor was 3 which had depth 0, and thus it
    // will be in the state `InCycleWith { 3 }`.
    //
    // When we finally traverse the `0 -> 4` edge and then visit node 2,
    // the states of the nodes are:
    //
    // 0 BeingVisited { 0 }
    // 1 InCycleWith { 3 }
    // 2 InCycleWith { 1 }
    // 3 InCycleWith { 0 }
    //
    // and hence 4 will traverse the links, finding an ultimate depth of 0.
    // If will also collapse the states to the following:
    //
    // 0 BeingVisited { 0 }
    // 1 InCycleWith { 3 }
    // 2 InCycleWith { 1 }
    // 3 InCycleWith { 0 }

    let sccs: UsizeSccs = Sccs::new(&graph);
    assert_eq!(sccs.num_sccs(), 1);
    assert_eq!(sccs.scc(0), 0);
    assert_eq!(sccs.scc(1), 0);
    assert_eq!(sccs.scc(2), 0);
    assert_eq!(sccs.scc(3), 0);
    assert_eq!(sccs.scc(4), 0);
    assert_eq!(sccs.successors(0), &[] as &[usize]);
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=test_find_state_3 | COMPLEXITY=3 | LINES=26

```rust
#[test]
fn test_find_state_3() {
    /*
          /----+
        0 <--+ |
        |    | |
        v    | |
    +-> 1 -> 3 4 5
    |   |      | |
    |   v      | |
    +-- 2 <----+-+
         */
    let graph =
        TestGraph::new(0, &[(0, 1), (0, 4), (1, 2), (1, 3), (2, 1), (3, 0), (4, 2), (5, 2)]);
    let sccs: UsizeSccs = Sccs::new(&graph);
    assert_eq!(sccs.num_sccs(), 2);
    assert_eq!(sccs.scc(0), 0);
    assert_eq!(sccs.scc(1), 0);
    assert_eq!(sccs.scc(2), 0);
    assert_eq!(sccs.scc(3), 0);
    assert_eq!(sccs.scc(4), 0);
    assert_eq!(sccs.scc(5), 1);
    assert_eq!(sccs.successors(0), &[] as &[usize]);
    assert_eq!(sccs.successors(1), &[0]);
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=test_deep_linear | COMPLEXITY=6 | LINES=29

```rust
#[test]
fn test_deep_linear() {
    /*
    0
    |
    v
    1
    |
    v
    2
    |
    v
    …
     */
    #[cfg(not(miri))]
    const NR_NODES: usize = 1 << 14;
    #[cfg(miri)]
    const NR_NODES: usize = 1 << 3;
    let mut nodes = vec![];
    for i in 1..NR_NODES {
        nodes.push((i - 1, i));
    }
    let graph = TestGraph::new(0, nodes.as_slice());
    let sccs: UsizeSccs = Sccs::new(&graph);
    assert_eq!(sccs.num_sccs(), NR_NODES);
    assert_eq!(sccs.scc(0), NR_NODES - 1);
    assert_eq!(sccs.scc(NR_NODES - 1), 0);
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=bench_sccc | COMPLEXITY=7 | LINES=44

```rust
#[bench]
fn bench_sccc(b: &mut test::Bencher) {
    // Like `test_three_sccs` but each state is replaced by a group of
    // three or four to have some amount of test data.
    /*
       0-3
        |
        v
    +->4-6 11-14
    |   |    |
    |   v    |
    +--7-10<-+
         */
    fn make_3_clique(slice: &mut [(usize, usize)], base: usize) {
        slice[0] = (base + 0, base + 1);
        slice[1] = (base + 1, base + 2);
        slice[2] = (base + 2, base + 0);
    }
    // Not actually a clique but strongly connected.
    fn make_4_clique(slice: &mut [(usize, usize)], base: usize) {
        slice[0] = (base + 0, base + 1);
        slice[1] = (base + 1, base + 2);
        slice[2] = (base + 2, base + 3);
        slice[3] = (base + 3, base + 0);
        slice[4] = (base + 1, base + 3);
        slice[5] = (base + 2, base + 1);
    }

    let mut graph = [(0, 0); 6 + 3 + 6 + 3 + 4];
    make_4_clique(&mut graph[0..6], 0);
    make_3_clique(&mut graph[6..9], 4);
    make_4_clique(&mut graph[9..15], 7);
    make_3_clique(&mut graph[15..18], 11);
    graph[18] = (0, 4);
    graph[19] = (5, 7);
    graph[20] = (11, 10);
    graph[21] = (7, 4);
    let graph = TestGraph::new(0, &graph[..]);
    b.iter(|| {
        let sccs: UsizeSccs = Sccs::new(&graph);
        assert_eq!(sccs.num_sccs(), 3);
    });
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=test_max_self_loop | COMPLEXITY=6 | LINES=8

```rust
#[test]
fn test_max_self_loop() {
    let graph = TestGraph::new(0, &[(0, 0)]);
    let mut annotations = Maxes(IndexVec::new(), |n| if n == 0 { 17 } else { 0 });
    Sccs::new_with_annotation(&graph, &mut annotations);
    assert_eq!(annotations.0[0], 17);
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=test_max_branch | COMPLEXITY=2 | LINES=10

```rust
#[test]
fn test_max_branch() {
    let graph = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (2, 4)]);
    let mut annotations = Maxes(IndexVec::new(), |n| n);
    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);
    assert_eq!(annotations.0[sccs.scc(0)], 4);
    assert_eq!(annotations.0[sccs.scc(1)], 3);
    assert_eq!(annotations.0[sccs.scc(2)], 4);
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=test_single_cycle_max | COMPLEXITY=2 | LINES=9

```rust
#[test]
fn test_single_cycle_max() {
    let graph = TestGraph::new(0, &[(0, 2), (2, 3), (2, 4), (4, 1), (1, 2)]);
    let mut annotations = Maxes(IndexVec::new(), |n| n);
    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);
    assert_eq!(annotations.0[sccs.scc(2)], 4);
    assert_eq!(annotations.0[sccs.scc(0)], 4);
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=test_double_cycle_max | COMPLEXITY=6 | LINES=11

```rust
#[test]
fn test_double_cycle_max() {
    let graph =
        TestGraph::new(0, &[(0, 1), (1, 2), (1, 4), (2, 3), (2, 4), (3, 5), (4, 1), (5, 4)]);
    let mut annotations = Maxes(IndexVec::new(), |n| if n == 5 { 2 } else { 1 });

    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);

    assert_eq!(annotations.0[sccs.scc(0)].0, 2);
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=test_bug_minimised | COMPLEXITY=6 | LINES=14

```rust
#[test]
fn test_bug_minimised() {
    let graph = TestGraph::new(0, &[(0, 3), (0, 1), (3, 2), (2, 3), (1, 4), (4, 5), (5, 4)]);
    let mut annotations = Maxes(IndexVec::new(), |n| match n {
        3 => 1,
        _ => 0,
    });

    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);
    assert_eq!(annotations.annotation(sccs.scc(2)), 1);
    assert_eq!(annotations.annotation(sccs.scc(1)), 0);
    assert_eq!(annotations.annotation(sccs.scc(4)), 0);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=test_bug_max_leak_minimised | COMPLEXITY=6 | LINES=15

```rust
#[test]
fn test_bug_max_leak_minimised() {
    let graph = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (3, 0), (3, 4), (4, 3)]);
    let mut annotations = Maxes(IndexVec::new(), |w| match w {
        4 => 1,
        _ => 0,
    });

    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);

    assert_eq!(annotations.annotation(sccs.scc(2)), 0);
    assert_eq!(annotations.annotation(sccs.scc(3)), 1);
    assert_eq!(annotations.annotation(sccs.scc(0)), 1);
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=test_bug_max_leak | COMPLEXITY=8 | LINES=61

```rust
#[test]
fn test_bug_max_leak() {
    let graph = TestGraph::new(
        8,
        &[
            (0, 0),
            (0, 18),
            (0, 19),
            (0, 1),
            (0, 2),
            (0, 7),
            (0, 8),
            (0, 23),
            (18, 0),
            (18, 12),
            (19, 0),
            (19, 25),
            (12, 18),
            (12, 3),
            (12, 5),
            (3, 12),
            (3, 21),
            (3, 22),
            (5, 13),
            (21, 3),
            (22, 3),
            (13, 5),
            (13, 4),
            (4, 13),
            (4, 0),
            (2, 11),
            (7, 6),
            (6, 20),
            (20, 6),
            (8, 17),
            (17, 9),
            (9, 16),
            (16, 26),
            (26, 15),
            (15, 10),
            (10, 14),
            (14, 27),
            (23, 24),
        ],
    );
    let mut annotations = Maxes::new(|w| match w {
        22 => 1,
        24 => 2,
        27 => 2,
        _ => 0,
    });
    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);

    assert_eq!(annotations.annotation(sccs.scc(2)), 0);
    assert_eq!(annotations.annotation(sccs.scc(7)), 0);
    assert_eq!(annotations.annotation(sccs.scc(8)), 2);
    assert_eq!(annotations.annotation(sccs.scc(23)), 2);
    assert_eq!(annotations.annotation(sccs.scc(3)), 2);
    assert_eq!(annotations.annotation(sccs.scc(0)), 2);
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=test_bug_max_zero_stick_shape | COMPLEXITY=7 | LINES=16

```rust
#[test]
fn test_bug_max_zero_stick_shape() {
    let graph = TestGraph::new(0, &[(0, 1), (1, 2), (2, 3), (3, 2), (3, 4)]);
    let mut annotations = Maxes::new(|w| match w {
        4 => 1,
        _ => 0,
    });
    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);

    assert_eq!(annotations.annotation(sccs.scc(0)), 1);
    assert_eq!(annotations.annotation(sccs.scc(1)), 1);
    assert_eq!(annotations.annotation(sccs.scc(2)), 1);
    assert_eq!(annotations.annotation(sccs.scc(3)), 1);
    assert_eq!(annotations.annotation(sccs.scc(4)), 1);
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=test_min_max_in | COMPLEXITY=4 | LINES=15

```rust
#[test]
fn test_min_max_in() {
    let graph = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (3, 0), (3, 4), (4, 3), (3, 5)]);
    let mut annotations = MinMaxes(IndexVec::new(), |w| MinMaxIn { min: w, max: w });
    let sccs = Sccs::new_with_annotation(&graph, &mut annotations);

    assert_eq!(annotations.annotation(sccs.scc(2)).min, 2);
    assert_eq!(annotations.annotation(sccs.scc(2)).max, 2);
    assert_eq!(annotations.annotation(sccs.scc(0)).min, 0);
    assert_eq!(annotations.annotation(sccs.scc(0)).max, 4);
    assert_eq!(annotations.annotation(sccs.scc(3)).min, 0);
    assert_eq!(annotations.annotation(sccs.scc(3)).max, 4);
    assert_eq!(annotations.annotation(sccs.scc(5)).min, 5);
}
```

---
*Generated by AST tracing system*
