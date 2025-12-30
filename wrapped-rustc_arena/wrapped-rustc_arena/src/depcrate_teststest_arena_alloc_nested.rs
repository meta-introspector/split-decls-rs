// Generated macro for test_arena_alloc_nested (function)
macro_rules! Depcrate_teststest_arena_alloc_nested {
() => {
// Module: crate::tests
// Provides: {"test_arena_alloc_nested"}
// Dependencies: {}
# [test] fn test_arena_alloc_nested () { struct Inner { value : u8 , } struct Outer < 'a > { inner : & 'a Inner , } enum EI < 'e > { I (Inner) , O (Outer < 'e >) , } struct Wrap < 'a > (TypedArena < EI < 'a > >) ; impl < 'a > Wrap < 'a > { fn alloc_inner < F : Fn () -> Inner > (& self , f : F) -> & Inner { match self . 0 . alloc (EI :: I (f ())) { EI :: I (i) => i , _ => panic ! ("mismatch") , } } fn alloc_outer < F : Fn () -> Outer < 'a > > (& self , f : F) -> & Outer < '_ > { match self . 0 . alloc (EI :: O (f ())) { EI :: O (o) => o , _ => panic ! ("mismatch") , } } } let arena = Wrap (TypedArena :: default ()) ; let result = arena . alloc_outer (| | Outer { inner : arena . alloc_inner (| | Inner { value : 10 }) }) ; assert_eq ! (result . inner . value , 10) ; }
};
}
