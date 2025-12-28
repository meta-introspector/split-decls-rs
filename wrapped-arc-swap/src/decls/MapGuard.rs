macro_rules! MapGuard {
    () => {
        # [doc (hidden)] # [derive (Copy , Clone , Debug)] pub struct MapGuard < G , F , T , R > { guard : G , projection : F , _t : PhantomData < fn (& T) -> & R > , }
    };
}

MapGuard!()