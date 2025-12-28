macro_rules! deps {
    () => {
        RefCnt!();
        CaS!();
        AsRaw!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        pub (crate) mod sealed { use super :: * ; use crate :: as_raw :: AsRaw ; pub trait Protected < T > : Borrow < T > { fn into_inner (self) -> T ; fn from_inner (ptr : T) -> Self ; } pub trait InnerStrategy < T : RefCnt > { type Protected : Protected < T > ; unsafe fn load (& self , storage : & AtomicPtr < T :: Base >) -> Self :: Protected ; unsafe fn wait_for_readers (& self , old : * const T :: Base , storage : & AtomicPtr < T :: Base >) ; } pub trait CaS < T : RefCnt > : InnerStrategy < T > { unsafe fn compare_and_swap < C : AsRaw < T :: Base > > (& self , storage : & AtomicPtr < T :: Base > , current : C , new : T ,) -> Self :: Protected ; } }
    };
}

sealed!()