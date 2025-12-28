macro_rules! ForEachFut {
    () => {
        # [doc = " Takes a future and maps it to another future via a closure"] # [derive (Debug)] pub struct ForEachFut < F , FutT , T , FutB > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = () > , { done : bool , count : Arc < AtomicUsize > , f : F , fut_t : Option < FutT > , fut_b : Option < FutB > , }
    };
}

ForEachFut!();