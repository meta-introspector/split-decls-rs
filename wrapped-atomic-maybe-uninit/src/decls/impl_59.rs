macro_rules! deps {
    () => {
        AtomicMaybeUninit!();
        Array!();
        AtomicLoad!();
        Align16!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < T : raw :: AtomicLoad + PartialEq + core :: fmt :: Debug > Array < T > { pub (crate) fn new (base : T , rng : & mut fastrand :: Rng) -> Self { Self { arr : Box :: new (Align16 ([AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) , AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (base)) ,])) , base , idx : rng . usize (3 ..= 6) , } } pub (crate) fn get (& self) -> & AtomicMaybeUninit < T > { & self . arr . 0 [self . idx] } pub (crate) fn set (& mut self , new : T) { self . arr . 0 [self . idx] = AtomicMaybeUninit :: < T > :: new (MaybeUninit :: new (new)) ; # [cfg (valgrind)] { mark_no_access (& self . arr . 0) ; if size_of :: < T > () <= 2 { if IMP_ARM_LINUX || cfg ! (target_arch = "s390x") { mark_aligned_defined (& self . arr . 0 [self . idx]) ; } else if cfg ! (all (target_arch = "powerpc64" , not (any (target_feature = "partword-atomics" , atomic_maybe_uninit_target_feature = "partword-atomics" ,)) ,)) { mark_aligned_undefined (& self . arr . 0 [self . idx]) ; } } mark_defined (& self . arr . 0 [self . idx]) ; } } # [track_caller] pub (crate) unsafe fn assert (& self) { # [cfg (valgrind)] mark_defined (& self . arr . 0) ; for i in (0 .. self . idx) . chain (self . idx + 1 .. self . arr . 0 . len ()) { assert_eq ! (unsafe { self . arr . 0 [i] . load (Ordering :: Relaxed) . assume_init () } , self . base , "value at index {i} has changed, but must not change other than value at index {}" , self . idx ,) ; } } }
    };
}

impl_59!();