# AST Trace: ../rust/library/coretests/tests/atomic.rs

Generated 25 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=bool_ | COMPLEXITY=2 | LINES=12

```rust
use core::sync::atomic::Ordering::SeqCst;
use core::sync::atomic::*;

#[test]
fn bool_() {
    let a = AtomicBool::new(false);
    assert_eq!(a.compare_exchange(false, true, SeqCst, SeqCst), Ok(false));
    assert_eq!(a.compare_exchange(false, true, SeqCst, SeqCst), Err(true));

    a.store(false, SeqCst);
    assert_eq!(a.compare_exchange(false, true, SeqCst, SeqCst), Ok(false));
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=bool_and | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn bool_and() {
    let a = AtomicBool::new(true);
    assert_eq!(a.fetch_and(false, SeqCst), true);
    assert_eq!(a.load(SeqCst), false);
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=bool_nand | COMPLEXITY=2 | LINES=13

```rust
#[test]
fn bool_nand() {
    let a = AtomicBool::new(false);
    assert_eq!(a.fetch_nand(false, SeqCst), false);
    assert_eq!(a.load(SeqCst), true);
    assert_eq!(a.fetch_nand(false, SeqCst), true);
    assert_eq!(a.load(SeqCst), true);
    assert_eq!(a.fetch_nand(true, SeqCst), true);
    assert_eq!(a.load(SeqCst), false);
    assert_eq!(a.fetch_nand(true, SeqCst), false);
    assert_eq!(a.load(SeqCst), true);
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=uint_and | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn uint_and() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_and(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 & 0x137f);
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=uint_nand | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn uint_nand() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_nand(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), !(0xf731 & 0x137f));
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=uint_or | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn uint_or() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_or(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 | 0x137f);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=uint_xor | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn uint_xor() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_xor(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 ^ 0x137f);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=uint_min | COMPLEXITY=2 | LINES=10

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn uint_min() {
    let x = AtomicUsize::new(0xf731);
    assert_eq!(x.fetch_min(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0x137f);
    assert_eq!(x.fetch_min(0xf731, SeqCst), 0x137f);
    assert_eq!(x.load(SeqCst), 0x137f);
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=uint_max | COMPLEXITY=2 | LINES=10

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn uint_max() {
    let x = AtomicUsize::new(0x137f);
    assert_eq!(x.fetch_max(0xf731, SeqCst), 0x137f);
    assert_eq!(x.load(SeqCst), 0xf731);
    assert_eq!(x.fetch_max(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731);
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=int_and | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn int_and() {
    let x = AtomicIsize::new(0xf731);
    assert_eq!(x.fetch_and(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 & 0x137f);
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=int_nand | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn int_nand() {
    let x = AtomicIsize::new(0xf731);
    assert_eq!(x.fetch_nand(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), !(0xf731 & 0x137f));
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=int_or | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn int_or() {
    let x = AtomicIsize::new(0xf731);
    assert_eq!(x.fetch_or(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 | 0x137f);
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=int_xor | COMPLEXITY=2 | LINES=7

```rust
#[test]
fn int_xor() {
    let x = AtomicIsize::new(0xf731);
    assert_eq!(x.fetch_xor(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731 ^ 0x137f);
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=int_min | COMPLEXITY=2 | LINES=10

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn int_min() {
    let x = AtomicIsize::new(0xf731);
    assert_eq!(x.fetch_min(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0x137f);
    assert_eq!(x.fetch_min(0xf731, SeqCst), 0x137f);
    assert_eq!(x.load(SeqCst), 0x137f);
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=int_max | COMPLEXITY=2 | LINES=10

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn int_max() {
    let x = AtomicIsize::new(0x137f);
    assert_eq!(x.fetch_max(0xf731, SeqCst), 0x137f);
    assert_eq!(x.load(SeqCst), 0xf731);
    assert_eq!(x.fetch_max(0x137f, SeqCst), 0xf731);
    assert_eq!(x.load(SeqCst), 0xf731);
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=ptr_add_null | COMPLEXITY=3 | LINES=17

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn ptr_add_null() {
    let atom = AtomicPtr::<i64>::new(core::ptr::null_mut());
    assert_eq!(atom.fetch_ptr_add(1, SeqCst).addr(), 0);
    assert_eq!(atom.load(SeqCst).addr(), 8);

    assert_eq!(atom.fetch_byte_add(1, SeqCst).addr(), 8);
    assert_eq!(atom.load(SeqCst).addr(), 9);

    assert_eq!(atom.fetch_ptr_sub(1, SeqCst).addr(), 9);
    assert_eq!(atom.load(SeqCst).addr(), 1);

    assert_eq!(atom.fetch_byte_sub(1, SeqCst).addr(), 1);
    assert_eq!(atom.load(SeqCst).addr(), 0);
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=ptr_add_data | COMPLEXITY=3 | LINES=26

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn ptr_add_data() {
    let num = 0i64;
    let n = &num as *const i64 as *mut _;
    let atom = AtomicPtr::<i64>::new(n);
    assert_eq!(atom.fetch_ptr_add(1, SeqCst), n);
    assert_eq!(atom.load(SeqCst), n.wrapping_add(1));

    assert_eq!(atom.fetch_ptr_sub(1, SeqCst), n.wrapping_add(1));
    assert_eq!(atom.load(SeqCst), n);
    let bytes_from_n = |b| n.wrapping_byte_add(b);

    assert_eq!(atom.fetch_byte_add(1, SeqCst), n);
    assert_eq!(atom.load(SeqCst), bytes_from_n(1));

    assert_eq!(atom.fetch_byte_add(5, SeqCst), bytes_from_n(1));
    assert_eq!(atom.load(SeqCst), bytes_from_n(6));

    assert_eq!(atom.fetch_byte_sub(1, SeqCst), bytes_from_n(6));
    assert_eq!(atom.load(SeqCst), bytes_from_n(5));

    assert_eq!(atom.fetch_byte_sub(5, SeqCst), bytes_from_n(5));
    assert_eq!(atom.load(SeqCst), n);
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=ptr_bitops | COMPLEXITY=3 | LINES=14

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn ptr_bitops() {
    let atom = AtomicPtr::<i64>::new(core::ptr::null_mut());
    assert_eq!(atom.fetch_or(0b0111, SeqCst).addr(), 0);
    assert_eq!(atom.load(SeqCst).addr(), 0b0111);

    assert_eq!(atom.fetch_and(0b1101, SeqCst).addr(), 0b0111);
    assert_eq!(atom.load(SeqCst).addr(), 0b0101);

    assert_eq!(atom.fetch_xor(0b1111, SeqCst).addr(), 0b0101);
    assert_eq!(atom.load(SeqCst).addr(), 0b1010);
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=ptr_bitops_tagging | COMPLEXITY=3 | LINES=28

```rust
#[test]
#[cfg(any(not(target_arch = "arm"), target_os = "linux"))] // Missing intrinsic in compiler-builtins
fn ptr_bitops_tagging() {
    #[repr(align(16))]
    struct Tagme(#[allow(dead_code)] u128);

    let tagme = Tagme(1000);
    let ptr = &tagme as *const Tagme as *mut Tagme;
    let atom: AtomicPtr<Tagme> = AtomicPtr::new(ptr);

    const MASK_TAG: usize = 0b1111;
    const MASK_PTR: usize = !MASK_TAG;

    assert_eq!(ptr.addr() & MASK_TAG, 0);

    assert_eq!(atom.fetch_or(0b0111, SeqCst), ptr);
    assert_eq!(atom.load(SeqCst), ptr.map_addr(|a| a | 0b111));

    assert_eq!(atom.fetch_and(MASK_PTR | 0b0010, SeqCst), ptr.map_addr(|a| a | 0b111));
    assert_eq!(atom.load(SeqCst), ptr.map_addr(|a| a | 0b0010));

    assert_eq!(atom.fetch_xor(0b1011, SeqCst), ptr.map_addr(|a| a | 0b0010));
    assert_eq!(atom.load(SeqCst), ptr.map_addr(|a| a | 0b1001));

    assert_eq!(atom.fetch_and(MASK_PTR, SeqCst), ptr.map_addr(|a| a | 0b1001));
    assert_eq!(atom.load(SeqCst), ptr);
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=static_init | COMPLEXITY=3 | LINES=15

```rust
static S_FALSE: AtomicBool = AtomicBool::new(false);
static S_TRUE: AtomicBool = AtomicBool::new(true);
static S_INT: AtomicIsize = AtomicIsize::new(0);
static S_UINT: AtomicUsize = AtomicUsize::new(0);

#[test]
fn static_init() {
    // Note that we're not really testing the mutability here but it's important
    // on Android at the moment (#49775)
    assert!(!S_FALSE.swap(true, SeqCst));
    assert!(S_TRUE.swap(false, SeqCst));
    assert!(S_INT.fetch_add(1, SeqCst) == 0);
    assert!(S_UINT.fetch_add(1, SeqCst) == 0);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=atomic_access_bool | COMPLEXITY=2 | LINES=17

```rust
#[test]
fn atomic_access_bool() {
    let mut atom = AtomicBool::new(false);

    assert_eq!(*atom.get_mut(), false);
    atom.store(true, SeqCst);
    assert_eq!(*atom.get_mut(), true);
    atom.fetch_or(false, SeqCst);
    assert_eq!(*atom.get_mut(), true);
    atom.fetch_and(false, SeqCst);
    assert_eq!(*atom.get_mut(), false);
    atom.fetch_nand(true, SeqCst);
    assert_eq!(*atom.get_mut(), true);
    atom.fetch_xor(true, SeqCst);
    assert_eq!(*atom.get_mut(), false);
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=atomic_alignment | COMPLEXITY=4 | LINES=32

```rust
#[test]
fn atomic_alignment() {
    #[cfg(target_has_atomic = "8")]
    assert_eq!(align_of::<AtomicBool>(), size_of::<AtomicBool>());
    #[cfg(target_has_atomic = "ptr")]
    assert_eq!(align_of::<AtomicPtr<u8>>(), size_of::<AtomicPtr<u8>>());
    #[cfg(target_has_atomic = "8")]
    assert_eq!(align_of::<AtomicU8>(), size_of::<AtomicU8>());
    #[cfg(target_has_atomic = "8")]
    assert_eq!(align_of::<AtomicI8>(), size_of::<AtomicI8>());
    #[cfg(target_has_atomic = "16")]
    assert_eq!(align_of::<AtomicU16>(), size_of::<AtomicU16>());
    #[cfg(target_has_atomic = "16")]
    assert_eq!(align_of::<AtomicI16>(), size_of::<AtomicI16>());
    #[cfg(target_has_atomic = "32")]
    assert_eq!(align_of::<AtomicU32>(), size_of::<AtomicU32>());
    #[cfg(target_has_atomic = "32")]
    assert_eq!(align_of::<AtomicI32>(), size_of::<AtomicI32>());
    #[cfg(target_has_atomic = "64")]
    assert_eq!(align_of::<AtomicU64>(), size_of::<AtomicU64>());
    #[cfg(target_has_atomic = "64")]
    assert_eq!(align_of::<AtomicI64>(), size_of::<AtomicI64>());
    #[cfg(target_has_atomic = "128")]
    assert_eq!(align_of::<AtomicU128>(), size_of::<AtomicU128>());
    #[cfg(target_has_atomic = "128")]
    assert_eq!(align_of::<AtomicI128>(), size_of::<AtomicI128>());
    #[cfg(target_has_atomic = "ptr")]
    assert_eq!(align_of::<AtomicUsize>(), size_of::<AtomicUsize>());
    #[cfg(target_has_atomic = "ptr")]
    assert_eq!(align_of::<AtomicIsize>(), size_of::<AtomicIsize>());
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=atomic_compare_exchange | COMPLEXITY=4 | LINES=26

```rust
#[test]
fn atomic_compare_exchange() {
    use Ordering::*;

    static ATOMIC: AtomicIsize = AtomicIsize::new(0);

    ATOMIC.compare_exchange(0, 1, Relaxed, Relaxed).ok();
    ATOMIC.compare_exchange(0, 1, Acquire, Relaxed).ok();
    ATOMIC.compare_exchange(0, 1, Release, Relaxed).ok();
    ATOMIC.compare_exchange(0, 1, AcqRel, Relaxed).ok();
    ATOMIC.compare_exchange(0, 1, SeqCst, Relaxed).ok();
    ATOMIC.compare_exchange(0, 1, Acquire, Acquire).ok();
    ATOMIC.compare_exchange(0, 1, AcqRel, Acquire).ok();
    ATOMIC.compare_exchange(0, 1, SeqCst, Acquire).ok();
    ATOMIC.compare_exchange(0, 1, SeqCst, SeqCst).ok();
    ATOMIC.compare_exchange_weak(0, 1, Relaxed, Relaxed).ok();
    ATOMIC.compare_exchange_weak(0, 1, Acquire, Relaxed).ok();
    ATOMIC.compare_exchange_weak(0, 1, Release, Relaxed).ok();
    ATOMIC.compare_exchange_weak(0, 1, AcqRel, Relaxed).ok();
    ATOMIC.compare_exchange_weak(0, 1, SeqCst, Relaxed).ok();
    ATOMIC.compare_exchange_weak(0, 1, Acquire, Acquire).ok();
    ATOMIC.compare_exchange_weak(0, 1, AcqRel, Acquire).ok();
    ATOMIC.compare_exchange_weak(0, 1, SeqCst, Acquire).ok();
    ATOMIC.compare_exchange_weak(0, 1, SeqCst, SeqCst).ok();
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=atomic_const_from | COMPLEXITY=2 | LINES=8

```rust
/* FIXME(#110395)
#[test]
fn atomic_const_from() {
    const _ATOMIC_U8: AtomicU8 = AtomicU8::from(1);
    const _ATOMIC_BOOL: AtomicBool = AtomicBool::from(true);
    const _ATOMIC_PTR: AtomicPtr<u32> = AtomicPtr::from(core::ptr::null_mut());
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=1

```rust
*/
```

---
*Generated by AST tracing system*
