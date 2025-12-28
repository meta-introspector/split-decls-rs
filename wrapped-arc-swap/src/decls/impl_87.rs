macro_rules! deps {
    () => {
        LocalNode!();
        RefCnt!();
        Debt!();
        Node!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Debt { # [doc = " Tries to pay the given debt."] # [doc = ""] # [doc = " If the debt is still there, for the given pointer, it is paid and `true` is returned. If it"] # [doc = " is empty or if there's some other pointer, it is not paid and `false` is returned, meaning"] # [doc = " the debt was paid previously by someone else."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " * It is possible that someone paid the debt and then someone else put a debt for the same"] # [doc = "   pointer in there. This is fine, as we'll just pay the debt for that someone else."] # [doc = " * This relies on the fact that the same pointer must point to the same object and"] # [doc = "   specifically to the same type ‒ the caller provides the type, it's destructor, etc."] # [doc = " * It also relies on the fact the same thing is not stuffed both inside an `Arc` and `Rc` or"] # [doc = "   something like that, but that sounds like a reasonable assumption. Someone storing it"] # [doc = "   through `ArcSwap<T>` and someone else with `ArcSwapOption<T>` will work."] # [inline] pub (crate) fn pay < T : RefCnt > (& self , ptr : * const T :: Base) -> bool { self . 0 . compare_exchange (ptr as usize , Self :: NONE , Release , Relaxed) . is_ok () } # [doc = " Pays all the debts on the given pointer and the storage."] pub (crate) fn pay_all < T , R > (ptr : * const T :: Base , storage_addr : usize , replacement : R) where T : RefCnt , R : Fn () -> T , { LocalNode :: with (| local | { let val = unsafe { T :: from_ptr (ptr) } ; T :: inc (& val) ; Node :: traverse :: < () , _ > (| node | { let _reservation = node . reserve_writer () ; local . help (node , storage_addr , & replacement) ; let all_slots = node . fast_slots () . chain (core :: iter :: once (node . helping_slot ())) ; for slot in all_slots { if slot . pay :: < T > (ptr) { T :: inc (& val) ; } } None }) ; }) } }
    };
}

impl_87!()