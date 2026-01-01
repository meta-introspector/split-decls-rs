/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: cell :: Cell ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_USE_0002
/* FP:tests.rs-0004 */ use test :: Bencher ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_USE_0003
/* FP:tests.rs-0006 */ use super :: TypedArena ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_STRUCT_0004
/* FP:tests.rs-0008 */ # [allow (dead_code)] # [derive (Debug , Eq , PartialEq)] struct Point { x : i32 , y : i32 , z : i32 , }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_IMPL_0005
/* FP:tests.rs-0010 */ impl < T > TypedArena < T > { # [doc = " Clears the arena. Deallocates all but the longest chunk which may be reused."] fn clear (& mut self) { unsafe { let mut chunks_borrow = self . chunks . borrow_mut () ; if let Some (mut last_chunk) = chunks_borrow . last_mut () { self . clear_last_chunk (& mut last_chunk) ; let len = chunks_borrow . len () ; for mut chunk in chunks_borrow . drain (.. len - 1) { chunk . destroy (chunk . entries) ; } } } } }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn test_unused () { let arena : TypedArena < Point > = TypedArena :: default () ; assert ! (arena . chunks . borrow () . is_empty ()) ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn test_arena_alloc_nested () { struct Inner { value : u8 , } struct Outer < 'a > { inner : & 'a Inner , } enum EI < 'e > { I (Inner) , O (Outer < 'e >) , } struct Wrap < 'a > (TypedArena < EI < 'a > >) ; impl < 'a > Wrap < 'a > { fn alloc_inner < F : Fn () -> Inner > (& self , f : F) -> & Inner { match self . 0 . alloc (EI :: I (f ())) { EI :: I (i) => i , _ => panic ! ("mismatch") , } } fn alloc_outer < F : Fn () -> Outer < 'a > > (& self , f : F) -> & Outer < '_ > { match self . 0 . alloc (EI :: O (f ())) { EI :: O (o) => o , _ => panic ! ("mismatch") , } } } let arena = Wrap (TypedArena :: default ()) ; let result = arena . alloc_outer (| | Outer { inner : arena . alloc_inner (| | Inner { value : 10 }) }) ; assert_eq ! (result . inner . value , 10) ; }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0008
/* FP:tests.rs-0016 */ # [test] fn test_copy () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; } }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0009
/* FP:tests.rs-0018 */ # [bench] fn bench_copy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | arena . alloc (Point { x : 1 , y : 2 , z : 3 })) }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0010
/* FP:tests.rs-0020 */ # [bench] fn bench_copy_nonarena (b : & mut Bencher) { b . iter (| | { let _ : Box < _ > = Box :: new (Point { x : 1 , y : 2 , z : 3 }) ; }) }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_STRUCT_0011
/* FP:tests.rs-0022 */ # [allow (dead_code)] struct Noncopy { string : String , array : Vec < i32 > , }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0012
/* FP:tests.rs-0024 */ # [test] fn test_noncopy () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) ; } }
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0013
/* FP:tests.rs-0026 */ # [test] fn test_typed_arena_zero_sized () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (()) ; } }
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0014
/* FP:tests.rs-0028 */ # [test] fn test_typed_arena_clear () { let mut arena = TypedArena :: default () ; for _ in 0 .. 10 { arena . clear () ; # [cfg (not (miri))] const N : usize = 10000 ; # [cfg (miri)] const N : usize = 100 ; for _ in 0 .. N { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; } } }
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0015
/* FP:tests.rs-0030 */ # [bench] fn bench_typed_arena_clear (b : & mut Bencher) { let mut arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; arena . clear () ; }) }
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0016
/* FP:tests.rs-0032 */ # [bench] fn bench_typed_arena_clear_100 (b : & mut Bencher) { let mut arena = TypedArena :: default () ; b . iter (| | { for _ in 0 .. 100 { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; } arena . clear () ; }) }
/* FP:tests.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_STRUCT_0017
/* FP:tests.rs-0034 */ struct DropCounter < 'a > { count : & 'a Cell < u32 > , }
/* FP:tests.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_IMPL_0018
/* FP:tests.rs-0036 */ impl Drop for DropCounter < '_ > { fn drop (& mut self) { self . count . set (self . count . get () + 1) ; } }
/* FP:tests.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0019
/* FP:tests.rs-0038 */ # [test] fn test_typed_arena_drop_count () { let counter = Cell :: new (0) ; { let arena : TypedArena < DropCounter < '_ > > = TypedArena :: default () ; for _ in 0 .. 100 { arena . alloc (DropCounter { count : & counter }) ; } } ; assert_eq ! (counter . get () , 100) ; }
/* FP:tests.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0020
/* FP:tests.rs-0040 */ # [test] fn test_typed_arena_drop_on_clear () { let counter = Cell :: new (0) ; let mut arena : TypedArena < DropCounter < '_ > > = TypedArena :: default () ; for i in 0 .. 10 { for _ in 0 .. 100 { arena . alloc (DropCounter { count : & counter }) ; } arena . clear () ; assert_eq ! (counter . get () , i * 100 + 100) ; } }
/* FP:tests.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_MACRO_0021
/* FP:tests.rs-0042 */ thread_local ! { static DROP_COUNTER : Cell < u32 > = Cell :: new (0) }
/* FP:tests.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_STRUCT_0022
/* FP:tests.rs-0044 */ struct SmallDroppable ;
/* FP:tests.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_IMPL_0023
/* FP:tests.rs-0046 */ impl Drop for SmallDroppable { fn drop (& mut self) { DROP_COUNTER . with (| c | c . set (c . get () + 1)) ; } }
/* FP:tests.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0024
/* FP:tests.rs-0048 */ # [test] fn test_typed_arena_drop_small_count () { DROP_COUNTER . with (| c | c . set (0)) ; { let arena : TypedArena < SmallDroppable > = TypedArena :: default () ; for _ in 0 .. 100 { arena . alloc (SmallDroppable) ; } } ; assert_eq ! (DROP_COUNTER . with (| c | c . get ()) , 100) ; }
/* FP:tests.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0025
/* FP:tests.rs-0050 */ # [bench] fn bench_noncopy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) }) }
/* FP:tests.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_arena_src_tests_FN_0026
/* FP:tests.rs-0052 */ # [bench] fn bench_noncopy_nonarena (b : & mut Bencher) { b . iter (| | { let _ : Box < _ > = Box :: new (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) ; }) }