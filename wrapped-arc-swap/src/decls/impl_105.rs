macro_rules! deps {
    () => {
        Debt!();
        HybridProtection!();
        RefCnt!();
        LocalNode!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < T : RefCnt > HybridProtection < T > { pub (super) unsafe fn new (ptr : * const T :: Base , debt : Option < & 'static Debt >) -> Self { Self { debt , ptr : ManuallyDrop :: new (T :: from_ptr (ptr)) , } } # [doc = " Try getting a dept into a fast slot."] # [inline] fn attempt (node : & LocalNode , storage : & AtomicPtr < T :: Base >) -> Option < Self > { let ptr = storage . load (Relaxed) ; let debt = node . new_fast (ptr as usize) ? ; let confirm = storage . load (SeqCst) ; if ptr == confirm { Some (unsafe { Self :: new (ptr , Some (debt)) }) } else if debt . pay :: < T > (ptr) { None } else { Some (unsafe { Self :: new (ptr , None) }) } } # [doc = " Get a debt slot using the slower but always successful mechanism."] fn fallback (node : & LocalNode , storage : & AtomicPtr < T :: Base >) -> Self { let gen = node . new_helping (storage as * const _ as usize) ; let candidate = storage . load (Acquire) ; match node . confirm_helping (gen , candidate as usize) { Ok (debt) => { Self :: from_inner (unsafe { Self :: new (candidate , Some (debt)) . into_inner () }) } Err ((unused_debt , replacement)) => { if ! unused_debt . pay :: < T > (candidate) { unsafe { T :: dec (candidate) } ; } unsafe { Self :: new (replacement as * mut _ , None) } } } } # [inline] fn as_ptr (& self) -> * const T :: Base { T :: as_ptr (self . ptr . deref ()) } }
    };
}

impl_105!()