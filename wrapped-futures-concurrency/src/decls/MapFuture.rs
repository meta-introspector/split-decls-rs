macro_rules! MapFuture {
    () => {
        # [doc = " Takes a future and maps it to another future via a closure"] # [derive (Debug)] pub struct MapFuture < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = B > , { done : bool , f : F , fut_t : Option < FutT > , fut_b : Option < FutB > , }
    };
}

MapFuture!()