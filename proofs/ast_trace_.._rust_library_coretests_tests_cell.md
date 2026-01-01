# AST Trace: ../rust/library/coretests/tests/cell.rs

Generated 41 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=smoketest_unsafe_cell | COMPLEXITY=7 | LINES=18

```rust
use core::cell::*;
use core::mem::forget;

#[test]
fn smoketest_unsafe_cell() {
    let mut x = UnsafeCell::new(10);
    let ref_mut = &mut x;
    unsafe {
        // The asserts are repeated in order to ensure that `get()`
        // is non-mutating.
        assert_eq!(*ref_mut.get(), 10);
        assert_eq!(*ref_mut.get(), 10);
        *ref_mut.get_mut() += 5;
        assert_eq!(*ref_mut.get(), 15);
        assert_eq!(*ref_mut.get(), 15);
        assert_eq!(x.into_inner(), 15);
    }
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=unsafe_cell_raw_get | COMPLEXITY=8 | LINES=16

```rust
#[test]
fn unsafe_cell_raw_get() {
    let x = UnsafeCell::new(10);
    let ptr = &x as *const UnsafeCell<i32>;
    unsafe {
        // The asserts are repeated in order to ensure that `raw_get()`
        // is non-mutating.
        assert_eq!(*UnsafeCell::raw_get(ptr), 10);
        assert_eq!(*UnsafeCell::raw_get(ptr), 10);
        *UnsafeCell::raw_get(ptr) += 5;
        assert_eq!(*UnsafeCell::raw_get(ptr), 15);
        assert_eq!(*UnsafeCell::raw_get(ptr), 15);
        assert_eq!(x.into_inner(), 15);
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=smoketest_cell | COMPLEXITY=2 | LINES=14

```rust
#[test]
fn smoketest_cell() {
    let x = Cell::new(10);
    assert_eq!(x, Cell::new(10));
    assert_eq!(x.get(), 10);
    x.set(20);
    assert_eq!(x, Cell::new(20));
    assert_eq!(x.get(), 20);

    let y = Cell::new((30, 40));
    assert_eq!(y, Cell::new((30, 40)));
    assert_eq!(y.get(), (30, 40));
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=cell_update | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn cell_update() {
    let x = Cell::new(10);

    x.update(|x| x + 5);
    assert_eq!(x.get(), 15);

    x.update(|x| x / 3);
    assert_eq!(x.get(), 5);
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=cell_has_sensible_show | COMPLEXITY=4 | LINES=9

```rust
#[test]
fn cell_has_sensible_show() {
    let x = Cell::new("foo bar");
    assert!(format!("{x:?}").contains(x.get()));

    x.set("baz qux");
    assert!(format!("{x:?}").contains(x.get()));
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=ref_and_refmut_have_sensible_show | COMPLEXITY=6 | LINES=15

```rust
#[test]
fn ref_and_refmut_have_sensible_show() {
    let refcell = RefCell::new("foo");

    let refcell_refmut = refcell.borrow_mut();
    assert_eq!(format!("{refcell_refmut}"), "foo"); // Display
    assert!(format!("{refcell_refmut:?}").contains("foo")); // Debug
    drop(refcell_refmut);

    let refcell_ref = refcell.borrow();
    assert_eq!(format!("{refcell_ref}"), "foo"); // Display
    assert!(format!("{refcell_ref:?}").contains("foo")); // Debug
    drop(refcell_ref);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=double_imm_borrow | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn double_imm_borrow() {
    let x = RefCell::new(0);
    let _b1 = x.borrow();
    x.borrow();
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=no_mut_then_imm_borrow | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn no_mut_then_imm_borrow() {
    let x = RefCell::new(0);
    let _b1 = x.borrow_mut();
    assert!(x.try_borrow().is_err());
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=no_imm_then_borrow_mut | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn no_imm_then_borrow_mut() {
    let x = RefCell::new(0);
    let _b1 = x.borrow();
    assert!(x.try_borrow_mut().is_err());
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=no_double_borrow_mut | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn no_double_borrow_mut() {
    let x = RefCell::new(0);
    assert!(x.try_borrow().is_ok());
    let _b1 = x.borrow_mut();
    assert!(x.try_borrow().is_err());
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=imm_release_borrow_mut | COMPLEXITY=3 | LINES=9

```rust
#[test]
fn imm_release_borrow_mut() {
    let x = RefCell::new(0);
    {
        let _b1 = x.borrow();
    }
    x.borrow_mut();
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=mut_release_borrow_mut | COMPLEXITY=3 | LINES=9

```rust
#[test]
fn mut_release_borrow_mut() {
    let x = RefCell::new(0);
    {
        let _b1 = x.borrow_mut();
    }
    x.borrow();
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=double_borrow_single_release_no_borrow_mut | COMPLEXITY=3 | LINES=11

```rust
#[test]
fn double_borrow_single_release_no_borrow_mut() {
    let x = RefCell::new(0);
    let _b1 = x.borrow();
    {
        let _b2 = x.borrow();
    }
    assert!(x.try_borrow().is_ok());
    assert!(x.try_borrow_mut().is_err());
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=discard_doesnt_unborrow | COMPLEXITY=2 | LINES=9

```rust
#[test]
#[should_panic]
fn discard_doesnt_unborrow() {
    let x = RefCell::new(0);
    let _b = x.borrow();
    let _ = _b;
    let _b = x.borrow_mut();
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=ref_clone_updates_flag | COMPLEXITY=5 | LINES=19

```rust
#[test]
fn ref_clone_updates_flag() {
    let x = RefCell::new(0);
    {
        let b1 = x.borrow();
        assert!(x.try_borrow().is_ok());
        assert!(x.try_borrow_mut().is_err());
        {
            let _b2 = Ref::clone(&b1);
            assert!(x.try_borrow().is_ok());
            assert!(x.try_borrow_mut().is_err());
        }
        assert!(x.try_borrow().is_ok());
        assert!(x.try_borrow_mut().is_err());
    }
    assert!(x.try_borrow().is_ok());
    assert!(x.try_borrow_mut().is_ok());
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=ref_map_does_not_update_flag | COMPLEXITY=5 | LINES=20

```rust
#[test]
fn ref_map_does_not_update_flag() {
    let x = RefCell::new(Some(5));
    {
        let b1: Ref<'_, Option<u32>> = x.borrow();
        assert!(x.try_borrow().is_ok());
        assert!(x.try_borrow_mut().is_err());
        {
            let b2: Ref<'_, u32> = Ref::map(b1, |o| o.as_ref().unwrap());
            assert_eq!(*b2, 5);
            assert!(x.try_borrow().is_ok());
            assert!(x.try_borrow_mut().is_err());
        }
        assert!(x.try_borrow().is_ok());
        assert!(x.try_borrow_mut().is_ok());
    }
    assert!(x.try_borrow().is_ok());
    assert!(x.try_borrow_mut().is_ok());
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=ref_map_split_updates_flag | COMPLEXITY=8 | LINES=37

```rust
#[test]
fn ref_map_split_updates_flag() {
    let x = RefCell::new([1, 2]);
    {
        let b1 = x.borrow();
        assert!(x.try_borrow().is_ok());
        assert!(x.try_borrow_mut().is_err());
        {
            let (_b2, _b3) = Ref::map_split(b1, |slc| slc.split_at(1));
            assert!(x.try_borrow().is_ok());
            assert!(x.try_borrow_mut().is_err());
        }
        assert!(x.try_borrow().is_ok());
        assert!(x.try_borrow_mut().is_ok());
    }
    assert!(x.try_borrow().is_ok());
    assert!(x.try_borrow_mut().is_ok());

    {
        let b1 = x.borrow_mut();
        assert!(x.try_borrow().is_err());
        assert!(x.try_borrow_mut().is_err());
        {
            let (_b2, _b3) = RefMut::map_split(b1, |slc| slc.split_at_mut(1));
            assert!(x.try_borrow().is_err());
            assert!(x.try_borrow_mut().is_err());
            drop(_b2);
            assert!(x.try_borrow().is_err());
            assert!(x.try_borrow_mut().is_err());
        }
        assert!(x.try_borrow().is_ok());
        assert!(x.try_borrow_mut().is_ok());
    }
    assert!(x.try_borrow().is_ok());
    assert!(x.try_borrow_mut().is_ok());
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=ref_map_split | COMPLEXITY=2 | LINES=8

```rust
#[test]
fn ref_map_split() {
    let x = RefCell::new([1, 2]);
    let (b1, b2) = Ref::map_split(x.borrow(), |slc| slc.split_at(1));
    assert_eq!(*b1, [1]);
    assert_eq!(*b2, [2]);
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=ref_mut_map_split | COMPLEXITY=3 | LINES=13

```rust
#[test]
fn ref_mut_map_split() {
    let x = RefCell::new([1, 2]);
    {
        let (mut b1, mut b2) = RefMut::map_split(x.borrow_mut(), |slc| slc.split_at_mut(1));
        assert_eq!(*b1, [1]);
        assert_eq!(*b2, [2]);
        b1[0] = 2;
        b2[0] = 1;
    }
    assert_eq!(*x.borrow(), [2, 1]);
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=ref_map_accessor | COMPLEXITY=4 | LINES=13

```rust
#[test]
fn ref_map_accessor() {
    struct X(RefCell<(u32, char)>);
    impl X {
        fn accessor(&self) -> Ref<'_, u32> {
            Ref::map(self.0.borrow(), |tuple| &tuple.0)
        }
    }
    let x = X(RefCell::new((7, 'z')));
    let d: Ref<'_, u32> = x.accessor();
    assert_eq!(*d, 7);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=ref_mut_map_accessor | COMPLEXITY=5 | LINES=17

```rust
#[test]
fn ref_mut_map_accessor() {
    struct X(RefCell<(u32, char)>);
    impl X {
        fn accessor(&self) -> RefMut<'_, u32> {
            RefMut::map(self.0.borrow_mut(), |tuple| &mut tuple.0)
        }
    }
    let x = X(RefCell::new((7, 'z')));
    {
        let mut d: RefMut<'_, u32> = x.accessor();
        assert_eq!(*d, 7);
        *d += 1;
    }
    assert_eq!(*x.0.borrow(), (8, 'z'));
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=as_ptr | COMPLEXITY=22 | LINES=23

```rust
#[test]
fn as_ptr() {
    let c1: Cell<usize> = Cell::new(0);
    c1.set(1);
    assert_eq!(1, unsafe { *c1.as_ptr() });

    let c2: Cell<usize> = Cell::new(0);
    unsafe {
        *c2.as_ptr() = 1;
    }
    assert_eq!(1, c2.get());

    let r1: RefCell<usize> = RefCell::new(0);
    *r1.borrow_mut() = 1;
    assert_eq!(1, unsafe { *r1.as_ptr() });

    let r2: RefCell<usize> = RefCell::new(0);
    unsafe {
        *r2.as_ptr() = 1;
    }
    assert_eq!(1, *r2.borrow());
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=cell_default | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn cell_default() {
    let cell: Cell<u32> = Default::default();
    assert_eq!(0, cell.get());
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=cell_set | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn cell_set() {
    let cell = Cell::new(10);
    cell.set(20);
    assert_eq!(20, cell.get());

    let cell = Cell::new("Hello".to_owned());
    cell.set("World".to_owned());
    assert_eq!("World".to_owned(), cell.into_inner());
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=cell_replace | COMPLEXITY=2 | LINES=11

```rust
#[test]
fn cell_replace() {
    let cell = Cell::new(10);
    assert_eq!(10, cell.replace(20));
    assert_eq!(20, cell.get());

    let cell = Cell::new("Hello".to_owned());
    assert_eq!("Hello".to_owned(), cell.replace("World".to_owned()));
    assert_eq!("World".to_owned(), cell.into_inner());
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=cell_into_inner | COMPLEXITY=2 | LINES=9

```rust
#[test]
fn cell_into_inner() {
    let cell = Cell::new(10);
    assert_eq!(10, cell.into_inner());

    let cell = Cell::new("Hello world".to_owned());
    assert_eq!("Hello world".to_owned(), cell.into_inner());
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=cell_exterior | COMPLEXITY=6 | LINES=24

```rust
#[test]
fn cell_exterior() {
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    struct Point {
        x: isize,
        y: isize,
        z: isize,
    }

    fn f(p: &Cell<Point>) {
        assert_eq!(p.get().z, 12);
        p.set(Point { x: 10, y: 11, z: 13 });
        assert_eq!(p.get().z, 13);
    }

    let a = Point { x: 10, y: 11, z: 12 };
    let b = &Cell::new(a);
    assert_eq!(b.get().z, 12);
    f(b);
    assert_eq!(a.z, 12);
    assert_eq!(b.get().z, 13);
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=cell_does_not_clone | COMPLEXITY=8 | LINES=23

```rust
#[test]
fn cell_does_not_clone() {
    #[derive(Copy)]
    #[allow(dead_code)]
    struct Foo {
        x: isize,
    }

    impl Clone for Foo {
        fn clone(&self) -> Foo {
            // Using Cell in any way should never cause clone() to be
            // invoked -- after all, that would permit evil user code to
            // abuse `Cell` and trigger crashes.

            panic!();
        }
    }

    let x = Cell::new(Foo { x: 22 });
    let _y = x.get();
    let _z = x.clone();
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=refcell_default | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn refcell_default() {
    let cell: RefCell<u64> = Default::default();
    assert_eq!(0, *cell.borrow());
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=unsafe_cell_unsized | COMPLEXITY=13 | LINES=12

```rust
#[test]
fn unsafe_cell_unsized() {
    let cell: &UnsafeCell<[i32]> = &UnsafeCell::new([1, 2, 3]);
    {
        let val: &mut [i32] = unsafe { &mut *cell.get() };
        val[0] = 4;
        val[2] = 5;
    }
    let comp: &mut [i32] = &mut [4, 2, 5];
    assert_eq!(unsafe { &mut *cell.get() }, comp);
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=refcell_unsized | COMPLEXITY=3 | LINES=12

```rust
#[test]
fn refcell_unsized() {
    let cell: &RefCell<[i32]> = &RefCell::new([1, 2, 3]);
    {
        let b = &mut *cell.borrow_mut();
        b[0] = 4;
        b[2] = 5;
    }
    let comp: &mut [i32] = &mut [4, 2, 5];
    assert_eq!(&*cell.borrow(), comp);
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=refcell_ref_coercion | COMPLEXITY=5 | LINES=18

```rust
#[test]
fn refcell_ref_coercion() {
    let cell: RefCell<[i32; 3]> = RefCell::new([1, 2, 3]);
    {
        let mut cellref: RefMut<'_, [i32; 3]> = cell.borrow_mut();
        cellref[0] = 4;
        let mut coerced: RefMut<'_, [i32]> = cellref;
        coerced[2] = 5;
    }
    {
        let comp: &mut [i32] = &mut [4, 2, 5];
        let cellref: Ref<'_, [i32; 3]> = cell.borrow();
        assert_eq!(&*cellref, comp);
        let coerced: Ref<'_, [i32]> = cellref;
        assert_eq!(&*coerced, comp);
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=refcell_swap_borrows | COMPLEXITY=2 | LINES=9

```rust
#[test]
#[should_panic]
fn refcell_swap_borrows() {
    let x = RefCell::new(0);
    let _b = x.borrow();
    let y = RefCell::new(1);
    x.swap(&y);
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=refcell_replace_borrows | COMPLEXITY=2 | LINES=8

```rust
#[test]
#[should_panic]
fn refcell_replace_borrows() {
    let x = RefCell::new(0);
    let _b = x.borrow();
    x.replace(1);
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=refcell_format | COMPLEXITY=4 | LINES=8

```rust
#[test]
fn refcell_format() {
    let name = RefCell::new("rust");
    let what = RefCell::new("rocks");
    let msg = format!("{name} {}", &*what.borrow(), name = &*name.borrow());
    assert_eq!(msg, "rust rocks".to_string());
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=const_cells | COMPLEXITY=3 | LINES=23

```rust
#[allow(dead_code)]
fn const_cells() {
    const UNSAFE_CELL: UnsafeCell<i32> = UnsafeCell::new(3);
    const _: i32 = UNSAFE_CELL.into_inner();

    const REF_CELL: RefCell<i32> = RefCell::new(3);
    const _: i32 = REF_CELL.into_inner();

    const CELL: Cell<i32> = Cell::new(3);
    const _: i32 = CELL.into_inner();

    /* FIXME(#110395)
        const UNSAFE_CELL_FROM: UnsafeCell<i32> = UnsafeCell::from(3);
        const _: i32 = UNSAFE_CELL.into_inner();

        const REF_CELL_FROM: RefCell<i32> = RefCell::from(3);
        const _: i32 = REF_CELL.into_inner();

        const CELL_FROM: Cell<i32> = Cell::from(3);
        const _: i32 = CELL.into_inner();
    */
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=refcell_borrow | COMPLEXITY=3 | LINES=14

```rust
#[test]
fn refcell_borrow() {
    // Check that `borrow` is usable at compile-time
    const {
        let a = RefCell::new(0);
        assert!(a.try_borrow().is_ok());
        assert!(a.try_borrow_mut().is_ok());
        let a_ref = a.borrow();
        assert!(*a_ref == 0);
        assert!(a.try_borrow().is_ok());
        assert!(a.try_borrow_mut().is_err());
    }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=refcell_borrow_mut | COMPLEXITY=5 | LINES=19

```rust
#[test]
fn refcell_borrow_mut() {
    // Check that `borrow_mut` is usable at compile-time
    const {
        let mut a = RefCell::new(0);
        {
            assert!(a.try_borrow().is_ok());
            assert!(a.try_borrow_mut().is_ok());
            let mut a_ref = a.borrow_mut();
            assert!(*a_ref == 0);
            *a_ref = 10;
            assert!(*a_ref == 10);
            assert!(a.try_borrow().is_err());
            assert!(a.try_borrow_mut().is_err());
        }
        assert!(*a.get_mut() == 10);
    };
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=NeverDrop; | COMPLEXITY=5 | LINES=6

```rust
struct NeverDrop;
impl Drop for NeverDrop {
    fn drop(&mut self) {
        panic!("should never be called");
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=refcell_replace | COMPLEXITY=3 | LINES=15

```rust
#[test]
fn refcell_replace() {
    // Check that `replace` is usable at compile-time
    const {
        let a = RefCell::new(0);
        assert!(a.replace(10) == 0);
        let a = a.into_inner();
        assert!(a == 10);

        let b = RefCell::new(NeverDrop);
        forget(b.replace(NeverDrop));
        forget(b)
    };
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=refcell_swap | COMPLEXITY=3 | LINES=17

```rust
#[test]
fn refcell_swap() {
    // Check that `swap` is usable at compile-time
    const {
        let (a, b) = (RefCell::new(31), RefCell::new(41));
        a.swap(&b);
        let (a, b) = (a.into_inner(), b.into_inner());
        assert!(a == 41);
        assert!(b == 31);

        let c = RefCell::new(NeverDrop);
        let d = RefCell::new(NeverDrop);
        c.swap(&d);
        forget((c, d));
    };
}
```

---
*Generated by AST tracing system*
