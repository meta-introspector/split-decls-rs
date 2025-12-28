macro_rules! deps {
    () => {
        AtomicEpoch!();
        Epoch!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl AtomicEpoch { # [doc = " Creates a new atomic epoch."] # [inline] pub (crate) fn new (epoch : Epoch) -> Self { let data = AtomicUsize :: new (epoch . data) ; Self { data } } # [doc = " Loads a value from the atomic epoch."] # [inline] pub (crate) fn load (& self , ord : Ordering) -> Epoch { Epoch { data : self . data . load (ord) , } } # [doc = " Stores a value into the atomic epoch."] # [inline] pub (crate) fn store (& self , epoch : Epoch , ord : Ordering) { self . data . store (epoch . data , ord) ; } # [doc = " Stores a value into the atomic epoch if the current value is the same as `current`."] # [doc = ""] # [doc = " The return value is a result indicating whether the new value was written and containing"] # [doc = " the previous value. On success this value is guaranteed to be equal to `current`."] # [doc = ""] # [doc = " This method takes two `Ordering` arguments to describe the memory"] # [doc = " ordering of this operation. `success` describes the required ordering for the"] # [doc = " read-modify-write operation that takes place if the comparison with `current` succeeds."] # [doc = " `failure` describes the required ordering for the load operation that takes place when"] # [doc = " the comparison fails. Using `Acquire` as success ordering makes the store part"] # [doc = " of this operation `Relaxed`, and using `Release` makes the successful load"] # [doc = " `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`"] # [doc = " and must be equivalent to or weaker than the success ordering."] # [inline] pub (crate) fn compare_exchange (& self , current : Epoch , new : Epoch , success : Ordering , failure : Ordering ,) -> Result < Epoch , Epoch > { match self . data . compare_exchange (current . data , new . data , success , failure) { Ok (data) => Ok (Epoch { data }) , Err (data) => Err (Epoch { data }) , } } }
    };
}

impl_91!()