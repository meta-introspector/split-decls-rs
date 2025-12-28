macro_rules! deps {
    () => {
        Access!();
        Map!();
        Guard!();
        MapGuard!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < A , F , T , R > Access < R > for Map < A , T , F > where A : Access < T > , F : Fn (& T) -> & R + Clone , { type Guard = MapGuard < A :: Guard , F , T , R > ; fn load (& self) -> Self :: Guard { let guard = self . access . load () ; MapGuard { guard , projection : self . projection . clone () , _t : PhantomData , } } }
    };
}

impl_22!()