macro_rules! deps {
    () => {
        Idx!();
        IdxRange!();
        RawIdx!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T > IdxRange < T > { # [doc = " Creates a new index range"] # [doc = " inclusive of the start value and exclusive of the end value."] # [doc = ""] # [doc = " ```"] # [doc = " let mut arena = la_arena::Arena::new();"] # [doc = " let a = arena.alloc(\"a\");"] # [doc = " let b = arena.alloc(\"b\");"] # [doc = " let c = arena.alloc(\"c\");"] # [doc = " let d = arena.alloc(\"d\");"] # [doc = ""] # [doc = " let range = la_arena::IdxRange::new(b..d);"] # [doc = " assert_eq!(&arena[range], &[\"b\", \"c\"]);"] # [doc = " ```"] pub fn new (range : Range < Idx < T > >) -> Self { Self { range : range . start . into_raw () . into () .. range . end . into_raw () . into () , _p : PhantomData } } # [doc = " Creates a new index range"] # [doc = " inclusive of the start value and end value."] # [doc = ""] # [doc = " ```"] # [doc = " let mut arena = la_arena::Arena::new();"] # [doc = " let foo = arena.alloc(\"foo\");"] # [doc = " let bar = arena.alloc(\"bar\");"] # [doc = " let baz = arena.alloc(\"baz\");"] # [doc = ""] # [doc = " let range = la_arena::IdxRange::new_inclusive(foo..=baz);"] # [doc = " assert_eq!(&arena[range], &[\"foo\", \"bar\", \"baz\"]);"] # [doc = ""] # [doc = " let range = la_arena::IdxRange::new_inclusive(foo..=foo);"] # [doc = " assert_eq!(&arena[range], &[\"foo\"]);"] # [doc = " ```"] pub fn new_inclusive (range : RangeInclusive < Idx < T > >) -> Self { Self { range : u32 :: from (range . start () . into_raw ()) .. u32 :: from (range . end () . into_raw ()) + 1 , _p : PhantomData , } } # [doc = " Returns whether the index range is empty."] # [doc = ""] # [doc = " ```"] # [doc = " let mut arena = la_arena::Arena::new();"] # [doc = " let one = arena.alloc(1);"] # [doc = " let two = arena.alloc(2);"] # [doc = ""] # [doc = " assert!(la_arena::IdxRange::new(one..one).is_empty());"] # [doc = " ```"] pub fn is_empty (& self) -> bool { self . range . is_empty () } # [doc = " Returns the start of the index range."] pub fn start (& self) -> Idx < T > { Idx :: from_raw (RawIdx :: from (self . range . start)) } # [doc = " Returns the end of the index range."] pub fn end (& self) -> Idx < T > { Idx :: from_raw (RawIdx :: from (self . range . end)) } }
    };
}

impl_37!();