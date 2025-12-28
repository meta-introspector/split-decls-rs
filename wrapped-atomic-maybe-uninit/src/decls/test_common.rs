macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
        Align16!();
    };
}

macro_rules! test_common {
    () => {
        deps!();
        macro_rules ! test_common { ($ ty : ident) => { paste :: paste ! { # [allow (clippy :: alloc_instead_of_core , clippy :: arithmetic_side_effects , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks ,)] mod [< test_common_ $ ty >] { use std :: { boxed :: Box , mem :: { self , MaybeUninit } , } ; # [cfg (atomic_maybe_uninit_no_strict_provenance)] use crate :: utils :: ptr :: MutPtrExt as _ ; use crate :: { tests :: helper ::*, AtomicMaybeUninit } ; # [test] fn assert_auto_traits () { fn _assert < T : Send + Sync + Unpin + std :: panic :: UnwindSafe + std :: panic :: RefUnwindSafe , > () { } _assert ::< AtomicMaybeUninit <$ ty >> () ; } # [test] fn accessor () { const INTO_INNER : MaybeUninit <$ ty > = { let a = AtomicMaybeUninit :: new (MaybeUninit :: new (10)) ; a . into_inner () } ; # [cfg (not (atomic_maybe_uninit_no_const_mut_refs))] const GET_MUT : AtomicMaybeUninit <$ ty > = { let mut a = AtomicMaybeUninit :: new (MaybeUninit :: uninit ()) ; let _ = unsafe { AtomicMaybeUninit :: from_ptr (a . as_ptr ()) } ; * a . get_mut () = MaybeUninit :: new (5) ; a } ; # [allow (clippy :: ptr_as_ptr)] unsafe { assert_eq ! (INTO_INNER . assume_init () , 10) ; # [cfg (not (atomic_maybe_uninit_no_const_mut_refs))] { assert_eq ! (GET_MUT . into_inner () . assume_init () , 5) ; } let mut a = AtomicMaybeUninit ::<$ ty >:: new (MaybeUninit :: new (10)) ; assert_eq ! (* a . get_mut () . as_mut_ptr () , 10) ; assert_eq ! (a . as_ptr () as * const () , & a as * const _ as * const ()) ; * a . get_mut () = MaybeUninit :: new (5) ; assert_eq ! (a . into_inner () . assume_init () , 5) ; let ptr : * mut Align16 < MaybeUninit <$ ty >> = Box :: into_raw (Box :: new (Align16 (MaybeUninit :: new (0)))) ; assert ! (ptr . addr () % mem :: align_of ::< AtomicMaybeUninit <$ ty >> () == 0) ; { let a = AtomicMaybeUninit ::<$ ty >:: from_ptr (ptr . cast ::< MaybeUninit <$ ty >> ()) ; * a . as_ptr () = MaybeUninit :: new (1) ; } assert_eq ! ((* ptr) . 0 . assume_init () , 1) ; drop (Box :: from_raw (ptr)) ; } } # [test] fn impls () { unsafe { let a = AtomicMaybeUninit ::<$ ty >:: from (MaybeUninit :: new (0)) ; let b = AtomicMaybeUninit ::<$ ty >:: from (0) ; assert_eq ! (std :: format ! ("{a:?}") , concat ! ("atomic_maybe_uninit::AtomicMaybeUninit<" , stringify ! ($ ty) , ">") ,) ; assert_eq ! (a . into_inner () . assume_init () , b . into_inner () . assume_init ()) ; } } } } } ; }
    };
}

test_common!()